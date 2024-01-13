section .data
    var1 dd 5
    var2 dd 10
    result dd 0

section .text
    global _start

_start:
    mov eax, [var1]
    mov ebx, [var2]
    call fn1
    mov ecx, eax

    mov edx, [var1]
    mov esi, ecx
    call fn2
    mov edi, eax

    xor ebp, ebp
for_outer:
    mov ebx, ecx
    mov eax, edi
    call fn1

    mov ecx, eax
    xor esi, esi
for_inner:
    mov eax, ebx
    mov ebx, ecx
    call fn2

    add esi, 1
    cmp esi, 2
    jl for_inner

    add ebp, 1
    cmp ebp, 3
    jl for_outer

    mov ebx, ecx
    mov eax, edi
    call fn1

    mov ecx, eax
    xor edx, edx
while_loop:
    mov eax, edi
    mov ebx, edx
    call fn2

    add edx, 1
    cmp eax, 100
    jl while_loop

    mov [result], eax

    ; Exit the program
    mov eax, 1
    xor ebx, ebx
    int 0x80

fn1:
    add eax, ebx
    ret

fn2:
    imul eax, ebx
    ret
