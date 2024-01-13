fn fn1(a: i32, b: i32) -> i32 {
    a + b
}

fn fn2(c: i32, d: i32) -> i32 {
    c * d
}

fn main() {
    let mut var1 = 5;
    let mut var2 = 10;
    let mut result = 0;

    let mut func_generic = |x, y| {
        let mut var3 = fn1(x, y);
        let mut var4 = fn2(x, var3);

        for _ in 0..3 {
            let mut var6 = fn1(var3, var4);
            let mut var7 = fn2(var3, var6);

            for _ in 0..2 {
                let mut var9 = fn1(var4, var7);
                let mut var10 = fn2(var4, var9);

                let mut var11 = fn1(var7, var10);
                let mut var12 = fn2(var7, var11);

                let mut var13 = fn1(var10, var12);
                let mut var14 = fn2(var10, var13);
            }
        }

        let mut var15 = fn1(var12, var14);
        let mut var16 = fn2(var12, var15);

        while var15 < 100 {
            let mut var17 = fn1(var14, var16);
            let mut var18 = fn2(var14, var17);
            var15 = fn1(var16, var18);
        }

        result = var15;
    };

    func_generic(var1, var2);

    println!("{}", result);
}
