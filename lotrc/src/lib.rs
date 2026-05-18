#[cfg(feature = "python")]
use pyo3::prelude::*;

pub mod audio;
pub mod bin;
pub mod level;
pub mod level_alt;
pub mod level_info;
pub mod lua_stuff;
pub mod pak;
pub mod pak_alt;
pub mod read_write;
pub mod shader;
pub mod types;

pub use flate2::Compression;

#[cfg(feature = "python")]
#[pymodule]
mod lotrc {
    use pyo3::prelude::*;

    #[pymodule]
    mod audio {
        #[pymodule_export]
        use crate::audio::{AudioTable, Header, Obj1, Obj2};
    }

    #[pymodule]
    mod bin {
        #[pymodule_export]
        use crate::bin::{AssetHandle, Header, Radiosity, Tex};
    }

    #[pymodule]
    mod level {
        #[pymodule_export]
        use crate::level::Level;
    }

    #[pymodule]
    mod level_alt {
        #[pymodule_export]
        use crate::level_alt::Level;
    }

    #[pymodule]
    mod level_info {
        #[pymodule_export]
        use crate::level_info::{GamemodeVal, Header, LevelInfo, LevelVal};
    }

    #[pymodule]
    mod pak_alt {
        #[pymodule_export]
        use crate::pak_alt::{
            Animation, AnimationEvent, BVTreeMesh, BVTreeMeshInfo, Block, BlockHeader1,
            BlockHeader2, BlockValA, BlockValB, Box, Capsule, ConvexVertices, ConvexVerticesInfo,
            Cylinder, HkConstraint, HkShape, HkShape0, HkShapeInfo, Mat, Model, Radiosity,
            RadiosityVal, Shape, ShapeExtra, ShapeExtraInfo, Sphere, TRS,
        };
    }

    #[pymodule]
    mod pak {
        #[pymodule_export]
        use crate::pak::{
            Animation, AnimationBlockInfo, AnimationInfo, BlockAVal, BoundingBox, BufferInfo,
            EffectInfo, FoliageInfo, GFXBlockInfo, Header, HkConstraint, HkConstraintData,
            HkConstraintInfo, HkShape, HkShapeInfo, IBuffInfo, IndexBuffer, LodMeshes, Mat1, Mat2,
            Mat3, Mat4, MatBase, MatExtra, Model, ModelInfo, Obj0, ObjA, PFieldInfo,
            RadiosityValsInfo, Shape, ShapeInfo, TextureInfo, VBuffInfo, VertexTypes,
        };
        use pyo3::prelude::*;

        #[pymodule]
        mod model {
            #[pymodule_export]
            use crate::pak::model::{BlockHeader, BlockVal};
        }

        #[pymodule]
        mod shape {
            #[pymodule_export]
            use crate::pak::shape::Header;
        }

        #[pymodule]
        mod animation {
            #[pymodule_export]
            use crate::pak::animation::{
                HkaSplineSkeletalAnimation, HkaSplineSkeletalAnimationFlags,
                HkaSplineSkeletalAnimationObj1, HkaSplineSkeletalAnimationObj1Types,
                HkaSplineSkeletalAnimationObj2, Obj3, Obj5Header, RotationPolar32,
                RotationQuantization, RotationStraight16, RotationThreeComp24, RotationThreeComp40,
                RotationThreeComp48, RotationUncompressed,
            };
        }
    }

    #[pymodule]
    mod shader {
        #[pymodule_export]
        use crate::shader::{Header, ShaderHeader, Shaders};
    }

    #[pymodule]
    mod types {
        #[pymodule_export]
        use crate::types::{
            anim_tables, compression, crc_string, decomp_lua, gltf, hash_string, recomp_lua,
            unluac, zip_, AtlasUVVal, BaseTypes, CrowdHeader, CrowdItem, CrowdVal, GameObj,
            GameObjs, GameObjsHeader, GameObjsObjHeader, GameObjsTypeField, GameObjsTypeHeader,
            Lua, SSAVal, Spray, SprayInstance, SprayVal, SubBlock, SubBlocksBlockHeader,
            SubBlocksHeader, SSA,
        };
    }
}
