import ctypes
import os

base_dir = os.path.dirname(os.path.abspath(__file__))

with open(f"{base_dir}/bindings.h", "r") as f:
    header = f.read()

lines = header.split("\n")

def parse_ty(vals):
    ty, *ptrs = [i for i in vals if i != 'const' and i != 'struct']
    assert(False not in [i == '*' or i == '*const' for i in ptrs])
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
unions = {}
missing = set()
i = 0
while i < len(lines):
    line = lines[i]
    if line.startswith("#") or line == '': 
        i += 1
    elif line.startswith("typedef struct"):
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

def create_class(name, fields, union=False):
    ret = f"""
class {name}(ctypes.Structure):
\t_fields_ = [
"""
    for n, ty in fields.items():
        n, ty = get_field(n, *ty)
        ret += f"\t\t(\"{n}\", {ty}),\n"
    if union:
        ret += f"\t\t(\"union\", {name[1:]}_union),\n\t]\n"
        ret += f"\t_anonymous_ = (\"union\",)\n"
    else:
        ret += "\t]\n"
    return ret

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
    arg_names = [i[0] for i in args]
    if len(args) > 0 and get_ty(*args[0][1]) == struct:
        arg_names[0] = "self"
    else:
        ret += "\t@staticmethod\n"
    names = ", ".join(arg_names)
    ret += f"\tdef {name}({names}):\n\t\t"
    if res != ('void', []):
        ret += "return "
    ret += f"lib.{fn}({names})\n"
    return ret

def add_property(name, ty, ptrs):
    if len(ptrs) > 0 or ty in non_ref_objs:
        return f"""\t@property
\tdef {name}(self):
\t\treturn self.contents.{name}
"""
    else:
        return f"""\t@property
\tdef {name}(self):
\t\treturn {get_ty(ty, ["*"])}(self.contents.{name})
"""

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
                k = "__del__"
            ret += create_method(name, v, k)
        if not union:
            for name, ty in fields.items():
                if name == 'align' or name.startswith('pad['): continue 
                ret += add_property(name, *ty)
        else:
            ret += union_get(name)
    return ret

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
    'u16': 'ctypes.c_uint16',
    'u32': 'ctypes.c_uint32',
    'u64': 'ctypes.c_uint64',
    'i16': 'ctypes.c_int16',
    'i32': 'ctypes.c_int32',
    'f32': 'ctypes.c_float',
}

bindings = """import ctypes
import os
base_dir = os.path.dirname(os.path.abspath(__file__))
lib = ctypes.cdll.LoadLibrary(f"{base_dir}/liblotrc_ffi.so")

try:
\timport numpy as np
\tdef get_array(x):
\t\tarr = np.ctypeslib.as_array(x.get(0), (x.len(),))
\t\tarr.setflags(write=False)
\t\treturn arr
except:
\tpass


"""

for ver in ['_le', '_be']:
    for k,v in ver_types.items():
        types[k+ver] = v
for name, ty in aliases.items():
    types[name] = types[ty]
for name, ty in enums.items():
    types[name] = types[ty[0]]
for ty in types:
    non_ref_objs.add(ty)

for name, ty in types.items():
    bindings += f"{name} = {ty}\n"

for struct in structs:
    if struct in non_ref_objs:
        types[struct] = struct
    else:
        types[struct] = "_" + struct;

for name, fields in structs.items():
    is_union = name in unions
    if is_union:
        bindings += create_union(name, unions[name])
    bindings += create_class(types[name], fields, is_union)
    if name not in non_ref_objs:
        bindings += create_ptr(name, fields, is_union)

for name, (ret, args) in functions.items():
   bindings += set_fn_info(name, ret, args)

for name in functions:
    if name not in seen_methods:
        bindings += f"{name} = lib.{name}\n"

with open(f"{base_dir}/lotrc_rs.py", "w") as f:
    f.write(bindings)
