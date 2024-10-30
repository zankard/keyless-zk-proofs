#pragma once

#include <cstdint>
#include <cstring>

static inline std::uint64_t NAFTable[1024];

inline void buildNaf(std::int64_t* r64, std::uint8_t* scalar,
                     unsigned int scalarSize)
{
    // r was uint8_t* which is not necessarly aligned same s int64_t
    // int64_t* r64 = (int64_t*)r;

    bool         carry = false;
    bool         last  = (scalar[0] & 1);
    std::int64_t rs;

    for (unsigned int i = 0; i < scalarSize + 2; i++)
    {
        int st = last ? 1 : 0;

        if (i < scalarSize)
        {
            st += scalar[i] & 0xFE;
        }

        if (i < scalarSize - 1)
        {
            st += (scalar[i + 1] & 1) << 8;
        }

        if (carry)
        {
            st += 0x200;
        }

        rs     = NAFTable[st];
        carry  = rs & 4;
        last   = rs & 8;
        r64[i] = rs & 0x0303030303030303LL;
    }
}

inline bool buildNafTable()
{
    for (int in = 0; in < 1024; in++)
    {
        bool         carry = (in & 0x200);
        bool         last  = (in & 1);
        std::uint8_t res[8];
        for (int i = 0; i < 8; i++)
        {
            bool cur = in & (1 << (i + 1));

            if (last)
            {
                if (cur)
                {
                    if (carry)
                    {
                        last   = false;
                        carry  = true;
                        res[i] = 1;
                    }
                    else
                    {
                        last   = false;
                        carry  = true;
                        res[i] = 2; // -1
                    }
                }
                else
                {
                    if (carry)
                    {
                        last   = false;
                        carry  = true;
                        res[i] = 2; // -1
                    }
                    else
                    {
                        last   = false;
                        carry  = false;
                        res[i] = 1;
                    }
                }
            }
            else
            {
                if (cur)
                {
                    if (carry)
                    {
                        last   = false;
                        carry  = true;
                        res[i] = 0;
                    }
                    else
                    {
                        last   = true;
                        carry  = false;
                        res[i] = 0;
                    }
                }
                else
                {
                    if (carry)
                    {
                        last   = true;
                        carry  = false;
                        res[i] = 0;
                    }
                    else
                    {
                        last   = false;
                        carry  = false;
                        res[i] = 0;
                    }
                }
            }
        }

        std::uint64_t r64;
        std::memcpy(&r64, res, sizeof(r64)); // = (*((int64_t*)(res)));
        if (carry)
        {
            r64 |= 0x4;
        }

        if (last)
        {
            r64 |= 0x8;
        }

        NAFTable[in] = r64;
    }
    return true;
}

static inline bool tableBulded = buildNafTable();
