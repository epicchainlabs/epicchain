; helper_functions.asm

section .text
    global add, subtract, multiply, divide, power, factorial

; Function: add
; Description: Adds two numbers.
; Parameters: eax - first number, ebx - second number
; Returns: eax - result
add:
    add eax, ebx
    ret

; Function: subtract
; Description: Subtracts two numbers.
; Parameters: eax - minuend, ebx - subtrahend
; Returns: eax - result
subtract:
    sub eax, ebx
    ret

; Function: multiply
; Description: Multiplies two numbers.
; Parameters: eax - first factor, ebx - second factor
; Returns: eax - result
multiply:
    imul ebx
    ret

; Function: divide
; Description: Divides two numbers.
; Parameters: eax - dividend, ebx - divisor
; Returns: eax - quotient
divide:
    xor edx, edx ; Clear edx for the division
    idiv ebx
    ret

; Function: power
; Description: Calculates the power of a number.
; Parameters: eax - base, ebx - exponent
; Returns: eax - result
power:
    mov ecx, ebx ; Move the exponent to ecx for the loop
    mov eax, 1 ; Initialize the result to 1
power_loop:
    test ecx, 1 ; Check if the lowest bit of ecx is set
    jz power_even ; Jump if the exponent is even
    imul eax, ebx ; Multiply the result by the base
power_even:
    shr ecx, 1 ; Divide the exponent by 2
    test ecx, ecx ; Check if the exponent is zero
    jnz power_loop ; Jump back to the loop if not zero
    ret

; Function: factorial
; Description: Calculates the factorial of a number.
; Parameters: eax - number
; Returns: eax - result
factorial:
    mov ecx, eax ; Copy the number to ecx for the loop
    mov eax, 1 ; Initialize the result to 1
factorial_loop:
    mul ecx ; Multiply the result by the current number in ecx
    loop factorial_loop ; Continue the loop until ecx is zero
    ret
