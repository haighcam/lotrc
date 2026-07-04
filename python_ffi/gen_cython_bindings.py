import os

base_dir = os.path.dirname(os.path.abspath(__file__))

with open(f"{base_dir}/lotrc.h", "r") as f:
    header = f.read()

lines = header.split("\n")

def parse_ty(vals):
    ty, *ptrs = [i for i in vals if i != 'const' and i != 'struct']
    assert(False not in [i == '*' or i == '*const' for i in ptrs])
    while (new_ty := struct_aliases.get(ty)) is not None:
        ty = new_ty
    return ty, ptrs

def parse_arg(arg):
    *tys, arg = arg.split()
    i = 0
    while arg[i] == '*':
        tys += ['*']
        i += 1
    return arg[i:], parse_ty(tys)

def parse_union(name, lines, i, unions):
    i += 1
    j = i
    fields = {}
    while not lines[i].endswith("};"):
        fname, ty = parse_arg(lines[i+1].strip())
        if fname[-1] == ";":
            fname = fname[:-1]
        fields[fname] = ty
        i += 3
    unions[name] = fields
    return i + 1

def parse_struct(lines, i, structs, unions, missing):
    if lines[i].endswith(";"):
        name, ty = parse_arg(lines[i][15:-1])
        if ty != [name]:
            structs[name] = {"val": ty}
        else:
            missing.add(name)
        return i + 1
    _, _, name, _ = lines[i].split()
    i += 1
    fields = {}
    while not lines[i].startswith('}'):
        if lines[i].endswith("union {"):
            i = parse_union(name, lines, i, unions)
        else:
            fname, ty = parse_arg(lines[i].strip())
            fields[fname[:-1]] = ty
            i += 1
    structs[name] = fields
    return i + 1

def parse_alias(lines, i, aliases):
    _, name, _, ty = lines[i].split()
    aliases[name] = ty[:-1]
    return i + 1

def parse_typedef(lines, i, aliases):
    name, (ty, ptrs) = parse_arg(lines[i][8:-1])
    assert(ptrs == [])
    print("typedef", name, ty, ptrs)
    aliases[name] = ty
    return i + 1

def parse_struct_typedef(lines, i, aliases, structs):
    name, (ty, ptrs) = parse_arg(lines[i][15:-1])
    assert(ptrs == [])
    if name == ty:
        structs[name] = {}
    else:
        print("typedef", name, ty, ptrs)
        aliases[name] = ty
    return i + 1

def parse_enum(lines, i, enums):
    # for now parse enum as alias
    _,name, _ = lines[i].split()
    i += 1
    vals = []
    while not lines[i].endswith('};'):
        vals.append(lines[i].strip()[:-1])
        i += 1
    i += 1
    _, ty, _ = lines[i].split()
    enums[name] = (ty, vals)
    return i + 1

def skip_comment(lines, i):
    if lines[i][:3] == '/**':
        while not lines[i].endswith('*/'):
            i += 1
    i += 1
    return i

def parse_function(lines, i, functions):
    line = lines[i]
    while not lines[i].endswith(");"):
        i += 1
        line = line + " " + lines[i].strip()
    ret_name, args = line.split('(')
    args = args[:-2].split(', ')
    name, ret_ty = parse_arg(ret_name)
    args = [parse_arg(i) for i in args if i != 'void']
    functions[name] = (ret_ty, args)
    return i + 1

structs = {}
enums = {}
functions = {}
aliases = {}
struct_aliases = {}
unions = {}
missing = set()
i = 0
while i < len(lines):
    line = lines[i]
    if line.startswith("#") or line == '': 
        i += 1
    elif line.startswith("typedef struct"):
        if line.endswith(";"):
            i = parse_struct_typedef(lines, i, struct_aliases, structs)
        else:
            i = parse_struct(lines, i, structs, unions, missing)
    elif line.startswith("enum"):
        i = parse_enum(lines, i, enums)
    elif line.startswith("using"):
        i = parse_alias(lines, i, aliases)
    elif line.startswith("typedef"):
        i = parse_typedef(lines, i, aliases)
    elif line.startswith("/"):
        i = skip_comment(lines, i)
    elif '(' in line:
        i = parse_function(lines, i, functions)
    else:
        print('aaa', line)

if len(missing) != 0:
    print("Missing Objects: ")
for i in sorted(missing):
    if i.endswith("Ps3") or i.endswith("Xbox"):
        continue
    if i.startswith("IndexMap"):
        print(f"make_indexmap_wrapper!({i}, u32, );")
    elif i.startswith("slice"):
        print(f"make_slice_wrapper!({i}, );")
    elif i.startswith("owned_slice"):
        print(f"make_owned_slice_wrapper!({i}, );")
    else:
        print(i)

non_ref_objs = set()
for a,b in functions.values():
    for t, p in [i[1] for i in b] + [a]:
        if len(p) == 0:
            non_ref_objs.add(t)
class_methods = {}
seen_methods = set()
for ty in structs:
    prefix = ty + "_"
    methods = {}
    for fn in functions:
        if fn.startswith(prefix):
            methods[fn.split(prefix)[-1]] = fn
            seen_methods.add(fn)
            assert(ty not in non_ref_objs)
    class_methods[ty] = methods

def get_ty(ty, ptrs):
    n_ptrs = 0
    if ty == 'void':
        if len(ptrs) > 0:
            n_ptrs = 1
            ty = 'ctypes.c_void_p'
        else:
            ty = 'None'
    elif ty not in non_ref_objs and len(ptrs) > 0:
        n_ptrs += 1
    else:
        ty = types[ty]
    while len(ptrs) > n_ptrs:
        n_ptrs += 1
        ty = f'ctypes.POINTER({ty})'
    return ty

def get_field(name, ty, ptrs):
    ty = get_ty(ty, ptrs)
    if name.endswith("]"):
        name, n = name.split("[")
        n = int(n[:-1])
        ty = f"{ty} * {n}"
    return (name, ty)

def create_union(name, fields):
    ret = f"""
class {name}_union(ctypes.Union):
\t_fields_ = [
"""
    for n, ty in fields.items():
        n, ty = get_field(n, *ty)
        ret += f"\t\t(\"{n}\", {ty}),\n"
    ret += "\t]\n"
    return ret

def create_method(struct, fn, name):
    ret = ""
    res, args = functions[fn]
    arg_names = []
    fn_arg_names = []
    is_static = True
    print(struct, name)
    for (i, arg) in enumerate(args):
        print("  ",arg, get_ty(*arg[1]))
        if arg[1][0] == struct and i == 0:
            fn_arg_names.append("self.ptr")
            arg_names.append("self")
            is_static = False
        elif arg[1][0] == "char":
            fn_arg_names.append(arg[0])
            arg_names.append(arg[0])
        elif arg[1][0] in basetype:
            fn_arg_names.append(("&" * len(arg[1][1])) + arg[0])
            #arg_names.append(arg[0])
            arg_names.append(f"lotrc_rs.{arg[1][0]} {arg[0]}")
        else:
            fn_arg_names.append(f"{arg[0]}.ptr")
            arg_names.append(f"{arg[1][0]} {arg[0]}")
    print(fn_arg_names, arg_names)
    if is_static:
        ret += "\t@staticmethod\n"
    names = ", ".join(arg_names)
    fnames = ", ".join(fn_arg_names)
    fn_call = f"lotrc_rs.{fn}({fnames})"
    ret += f"\tdef {name}({names}):\n\t\t"
    if res[0] not in basetype:
        assert(len(res[1]) > 0 and len(res[1]) < 3)
        if len(res[1]) == 2:
            fn_call = f"dereference({fn_call})"
        ret += f"""val = {res[0]}()
\t\tval.ptr = {fn_call}
\t\treturn val
"""
    elif res == ('void', []):
        ret += fn_call + "\n"
    elif res[1] != []:
        ret += f"return dereference({fn_call})\n"
    else:
        ret += f"return {fn_call}\n"
    return ret

def add_property(name, ty, ptrs):
#    if len(ptrs) > 0 or ty in non_ref_objs:
#        return f"""\t@property
#\tdef {name}(self):
#\t\treturn self.ptr.{name}
#"""
#    else:
#        return f"""\t@property
#\tdef {name}(self):
#\t\treturn {get_ty(ty, ["*"])}(self.ptr.{name})
#"""
    if ty in struct_aliases:
        ty = types[ty]
    ret = ""
    res, args = functions[fn]
    arg_names = ["self"]
    fn_arg_names = ["self.ptr"]
    ret += "\t@property\n"
    names = ", ".join(arg_names)
    fnames = ", ".join(fn_arg_names)
    fn_call = f"self.ptr.{name}"
    ret += f"\tdef {name}({names}):\n\t\t"
    if ty not in basetype:
        if len(ptrs) == 0:
            fn_call = f"&{fn_call}"
        ret += f"""val = {ty}()
\t\tval.ptr = {fn_call}
\t\treturn val
"""
    elif res == ('void', []):
        ret += fn_call + "\n"
    elif ptrs != []:
        ret += f"return dereference({fn_call})\n"
    else:
        ret += f"return {fn_call}\n"
    return ret

def union_get(name):
    ret = """\tdef get(self):
\t\ttag = self.contents.tag
"""
    for i, (n, (ty, ptrs)) in enumerate(unions[name].items()):
        if ty not in non_ref_objs and ptrs == []:
            ptrs = ["*"]
        ret += f"\t\t{'' if i == 0 else 'el'}if tag == {i}: return {get_ty(ty, ptrs)}(self.contents.{n})\n"
    return ret

def create_ptr(name, fields, union):
    ret = f"""
class {name}(ctypes.POINTER(_{name})):
\t_type_ = _{name}
"""
    if name in class_methods:
        for k, v in class_methods[name].items():
            if k == "free":
                k = "__dealloc__"
            ret += create_method(name, v, k)
        if not union:
            for name, ty in fields.items():
                if name == 'align' or name.startswith('pad['): continue 
                ret += add_property(name, *ty)
        else:
            ret += union_get(name)
    return ret

def create_class(name, fields, union):
    ret = f"""cdef class {name}:
\tcdef lotrc_rs.{name}* ptr
"""
    for k,v in class_methods.get(name, {}).items():
        if k == "free":
            k = "__dealloc__"
        ret += create_method(name, v, k)
    if not union:
        for name, ty in fields.items():
            if name == 'align' or name.startswith('pad['): continue 
            ret += add_property(name, *ty)
    return ret + "\n"

def set_fn_info(name, ret, args):
    ret = get_ty(*ret)
    if args == [('void', [])]:
        args_s = "[]"
    else:
        args_s = "[\n"
        for _, arg in args:
            args_s += f"\t{get_ty(*arg)},\n"
        args_s += "]"
    return f"""
lib.{name}.argtypes = {args_s}
lib.{name}.restype = {ret}
"""

basetype = set([
    'size_t',
    'uint16_t',
    'uint32_t',
    'uint64_t',
    'int16_t',
    'int32_t',
    'uint8_t',
    'bool',
    'char',
    'float',
    'uintptr_t',
    'void'
])

types = {
    'size_t': 'ctypes.c_size_t',
    'uint16_t': 'ctypes.c_uint16',
    'uint32_t': 'ctypes.c_uint32',
    'uint64_t': 'ctypes.c_uint64',
    'int16_t': 'ctypes.c_uint16',
    'int32_t': 'ctypes.c_int32',
    'uint8_t': 'ctypes.c_uint8',
    'bool': 'ctypes.c_bool',
    'char': 'ctypes.c_char',
    'float': 'ctypes.c_float',
    'uintptr_t': 'ctypes.c_size_t'
}

ver_types = {
    'u8':  'ctypes.c_uint8',
    'u16': 'ctypes.c_uint16',
    'u32': 'ctypes.c_uint32',
    'u64': 'ctypes.c_uint64',
    'i16': 'ctypes.c_int16',
    'i32': 'ctypes.c_int32',
    'I32': 'ctypes.c_int32',
    'f32': 'ctypes.c_float',
    'U32': 'ctypes.c_uint32',
    'Crc': 'ctypes.c_uint32',
}

bindings = """cimport lotrc_rs
from cython.operator import dereference

"""

for ty in enums:
    print(ty)
    basetype.add(ty)

for ver in ['_le', '_be']:
    for k,v in ver_types.items():
        types[k+ver] = v
        basetype.add(k+ver)
for plat in ['Pc', 'Xbox', 'Ps3']:
    for k in ver_types.keys():
        basetype.add(k+plat)
print(basetype)
for struct in structs:
    if struct in non_ref_objs:
        types[struct] = struct
    else:
        types[struct] = "_" + struct;
for name, ty in aliases.items():
    types[name] = types[ty]
for name, ty in struct_aliases.items():
    types[name] = types[ty]
for name, ty in enums.items():
    types[name] = types[ty[0]]
for ty in types:
    non_ref_objs.add(ty)

#for name, ty in types.items():
#    bindings += f"{name} = {ty}\n"


for name, fields in structs.items():
    is_union = name in unions
    #if is_union:
    #    bindings += create_union(name, unions[name])
    #bindings += create_class(types[name], fields, is_union)
    #if name not in non_ref_objs:
    #    bindings += create_ptr(name, fields, is_union)
    bindings += create_class(name, fields, is_union)


#for name, (ret, args) in functions.items():
#   bindings += set_fn_info(name, ret, args)

#for name in functions:
#    if name not in seen_methods:
#        bindings += f"{name} = lib.{name}\n"

with open(f"{base_dir}/lotrc.pyx", "w") as f:
    f.write(bindings)
