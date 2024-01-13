package main

import "fmt"

func fn1(a, b int) int {
    return a + b
}

func fn2(c, d int) int {
    return c * d
}

func main() {
    var1 := 5
    var2 := 10
    var result int

    funcGeneric := func(x, y int) {
        var3 := fn1(x, y)
        var4 := fn2(x, var3)

        for i := 0; i < 3; i++ {
            var6 := fn1(var3, var4)
            var7 := fn2(var3, var6)

            for j := 0; j < 2; j++ {
                var9 := fn1(var4, var7)
                var10 := fn2(var4, var9)

                var11 := fn1(var7, var10)
                var12 := fn2(var7, var11)

                var13 := fn1(var10, var12)
                var14 := fn2(var10, var13)
            }
        }

        var15 := fn1(var12, var14)
        var16 := fn2(var12, var15)

        for var15 < 100 {
            var17 := fn1(var14, var16)
            var18 := fn2(var14, var17)
            var15 = fn1(var16, var18)
        }

        result = var15
    }

    funcGeneric(var1, var2)

    fmt.Println(result)
}
