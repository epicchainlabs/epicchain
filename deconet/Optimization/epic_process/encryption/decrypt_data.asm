; decrypt_data.asm

section .data
    key db "secretkey", 0 ; Predefined decryption key
    encrypted_data db "encrypteddata", 0 ; Encrypted data to be decrypted
    decrypted_data db 256 dup(0) ; Buffer to store decrypted data

section .text
    global decrypt_data

decrypt_data:
    ; Parameters: 
    ;   Input: encrypted_data_ptr
    ;   Output: decrypted_data_ptr

    ; Decrypt the data using the predefined key
    mov esi, encrypted_data ; Pointer to the encrypted data
    mov edi, decrypted_data ; Pointer to the buffer for decrypted data
    mov ebx, key ; Pointer to the decryption key
    xor ecx, ecx ; Clear ECX register
decrypt_loop:
    mov al, [esi + ecx] ; Get a character from the encrypted data
    xor al, [ebx + ecx] ; XOR it with the corresponding character from the key
    mov [edi + ecx], al ; Store the decrypted character in the buffer
    inc ecx ; Move to the next character
    cmp al, 0 ; Check if we've reached the end of the data
    jnz decrypt_loop ; If not, continue decrypting

    ; Return the pointer to the decrypted data
    mov eax, edi ; Set return value to the pointer to the decrypted data
    ret
