; encrypt_data.asm

section .data
    key db "secretkey", 0 ; Predefined encryption key
    plaintext_data db "plaintextdata", 0 ; Plaintext data to be encrypted
    encrypted_data db 256 dup(0) ; Buffer to store encrypted data

section .text
    global encrypt_data

encrypt_data:
    ; Parameters: 
    ;   Input: plaintext_data_ptr
    ;   Output: encrypted_data_ptr

    ; Encrypt the data using the predefined key
    mov esi, plaintext_data ; Pointer to the plaintext data
    mov edi, encrypted_data ; Pointer to the buffer for encrypted data
    mov ebx, key ; Pointer to the encryption key
    xor ecx, ecx ; Clear ECX register
encrypt_loop:
    mov al, [esi + ecx] ; Get a character from the plaintext data
    xor al, [ebx + ecx] ; XOR it with the corresponding character from the key
    mov [edi + ecx], al ; Store the encrypted character in the buffer
    inc ecx ; Move to the next character
    cmp al, 0 ; Check if we've reached the end of the data
    jnz encrypt_loop ; If not, continue encrypting

    ; Return the pointer to the encrypted data
    mov eax, edi ; Set return value to the pointer to the encrypted data
    ret
