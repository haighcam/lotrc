#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

constexpr static const uint32_t LodInfo_UNKNOWN = 1;

constexpr static const uint32_t LodInfo_STATIC = 2;

constexpr static const uint32_t LodInfo_SKINNED = 4;

constexpr static const uint32_t LodInfo_PHYSICS = 8;

constexpr static const uint32_t LodInfo_BREAKABLE = 16;

constexpr static const uint32_t LodInfo_LOD0 = 32;

constexpr static const uint32_t LodInfo_LOD1 = 64;

constexpr static const uint32_t LodInfo_LOD2 = 128;

constexpr static const uint32_t LodInfo_LOD3 = 256;
