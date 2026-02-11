# Task 4. Password Cracker

## Situation  
You have a ZIP file protected by a 4-digit PIN code (0000–9999). You need to brute-force the password.

---

## What you need to do  

### 1. Validation function  
Write a function `try_password(password)` that:

- Checks whether the password is correct (for testing purposes, the correct password is `"1234"`).  
- If the password is correct, prints `Password found: 1234` and returns `True`.

---

### 2. Single-threaded brute force  

- Iterate through **all passwords from 0000 to 9999 in a single thread**.  
- Measure execution time.

---

### 3. Multi-threaded brute force  

Split the same range into 4 parts:

- Thread 1: `0000–2499`  
- Thread 2: `2500–4999`  
- Thread 3: `5000–7499`  
- Thread 4: `7500–9999`  

---

### 4. Thread stopping mechanism  

- As soon as one thread finds the password, stop all other threads.  
- Use `threading.Event()` for synchronization.

---

## Expected result  

- The 4-thread version runs faster than the single-threaded version.  
- Once the password is found, the program stops brute-forcing immediately.
