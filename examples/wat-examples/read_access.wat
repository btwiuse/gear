(module
    (import "env" "memory" (memory 0))
    (import "env" "alloc" (func $alloc (param i32) (result i32)))
    (export "init" (func $init))
    (func $init
        (local $i i32)

        ;; alloc 0x100 pages, so mem pages are: 0..=0xff
        (block
            i32.const 0x100
            call $alloc
            i32.eqz
            br_if 0
            unreachable
        )

        ;; make read for all gear pages
        (loop
            local.get $i
            i32.const 1
            i32.add
            local.set $i

            local.get $i
            i32.const 0x1000
            i32.mul
            i32.load
            drop

            local.get $i
            i32.const 0xfff
            i32.ne
            br_if 0
        )
    )
)
