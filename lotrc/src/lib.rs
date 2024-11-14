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
pub mod types;

#[pymodule]
mod lotrc {
    use pyo3::prelude::*;

    #[pymodule(submodule)]
    mod audio {
        #[pymodule_export]
        use crate::audio::{
            Header, Obj1, Obj2, AudioTable
        };
    }

    #[pymodule(submodule)]
    mod bin {
        #[pymodule_export]
        use crate::bin::{
            Header, Radiosity, Tex, AssetHandle
        };
    }

    #[pymodule(submodule)]
    mod level {
        #[pymodule_export]
        use crate::level::Level;
    }

    #[pymodule(submodule)]
    mod level_alt {
        #[pymodule_export]
        use crate::level_alt::Level;
    }

    #[pymodule(submodule)]
    mod level_info {
        #[pymodule_export]
        use crate::level_info::{
            Header, GamemodeVal, LevelVal, LevelInfo
        };
    }

    #[pymodule(submodule)]
    mod pak_alt {
        #[pymodule_export]
        use crate::pak_alt::{
            Shape, ShapeExtra, HkShape, Animation, Mat, HkConstraint, Model, HkShapeInfo, TRS, 
            HkShape0, Box, Sphere, Capsule, Cylinder, ConvexVertices, BVTreeMesh, ConvexVerticesInfo, BVTreeMeshInfo,
            ShapeExtraInfo
        };
    }

    #[pymodule(submodule)]
    mod pak {
        use pyo3::prelude::*;
        #[pymodule_export]
        use crate::pak::{
            Shape, HkShape, Animation, Mat1, HkConstraint, Model, HkShapeInfo, AnimationInfo, AnimationBlockInfo,
            HkConstraintInfo, HkConstraintData, ModelInfo, Mat2, Mat3, Mat4, VBuffInfo, IBuffInfo, Header,
            ObjA, Obj0, LodMeshes, BufferInfo, MatBase, MatExtra, ShapeInfo, TextureInfo, EffectInfo, PFieldInfo, 
            GFXBlockInfo, FoliageInfo, RadiosityValsInfo, BlockAVal, VertexUsage, VertexTypes, 
            VertexData, IndexBuffer
        };

        #[pymodule(submodule)]
        mod model {
            #[pymodule_export]
            use crate::pak::model::{BlockHeader, BlockVal};
        }

        #[pymodule(submodule)]
        mod shape {
            #[pymodule_export]
            use crate::pak::shape::Header;
        }

        #[pymodule(submodule)]
        mod animation {
            #[pymodule_export]
            use crate::pak::animation::{
                HkaSplineSkeletalAnimationObj1Types, HkaSplineSkeletalAnimationObj2Types,
                HkaSplineSkeletalAnimationObj1, HkaSplineSkeletalAnimationObj2,
                HkaSplineSkeletalAnimationObj2Type1, HkaSplineSkeletalAnimationObj2Type2,
                HkaSplineSkeletalAnimationObj2Type3, HkaSplineSkeletalAnimationObj2Type4,
                HkaSplineSkeletalAnimationObj2Type5, HkaSplineSkeletalAnimationObj2Type6,
                HkaSplineSkeletalAnimationFlags, HkaSplineSkeletalAnimation, Obj5Header, Obj3,
            };
        }
    }

    #[pymodule(submodule)]
    mod types {
        #[pymodule_export]
        use crate::types::{
            BaseTypes, SubBlocks, SubBlock, Spray, GameObjs, Lua, SSA, SubBlocksHeader,
            SubBlocksBlockHeader, StringKeysHeader, StringKeysVal, StringKeys, SSAVal, GameObjsHeader,
            GameObjsTypeHeader, GameObjsTypeField, GameObjsObjHeader, SprayInstance, SprayVal, 
            CrowdItem, CrowdHeader, CrowdVal, AtlasUVVal, hash_string
        };
    }
}
