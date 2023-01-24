# cipher_implementations_rust

Project contains the rust implementation's of all the cryptography methods I learn.
List of ciphers I will be implementing are:

- [ ] Playfair
- [x] Hill Cipher
- [x] Caesar Cipher
- [x] Keyword Cipher: Keyword cipher is a form of monoalphabetic substitution. A keyword is used as the key, and it
      determines the letter matchings of the cipher alphabet to the plain alphabet. Repeats of letters in the word are
      removed, then the cipher alphabet is generated with the keyword matching to A, B, C, etc. until the keyword is
      used up, whereupon the rest of the ciphertext letters are used in alphabetical order, excluding those already used in the key.

  _Implementation_

  To create a substitution alphabet from a keyword, you first write down the alphabet.
  Below this you write down the keyword (omitting duplicate letters) followed by the remaining unused letters of the alphabet

        ABCDEFGHIJKLMNOPQRSTUVWXYZ
        KEYWORDABCFGHIJLMNPQSTUVXZ

  To encipher a plaintext message, you convert all letters from the top row to their corresponding letter on the bottom row (A to K, B to E, etc).

  > Ex: plaintext = "Hello world" keyword = "secret"

  > encrypted = "dtiil wloir"

- [x] Affine Cipher
      The affine cipher is a type of monoalphabetic substitution cipher,which has 2 keys. Let us consider Key 1(a) and key 2 (b) and plaintext(x)
      when encrypting we use `a*X+b mod 26` to encrypt the message. While we can use `a(inverse)*(y-b) mod 26` to decrypt the encrypted message

  > Ex: Key = Hello, World!, Key 1 = 5, key 2 = 8

  > encryption = Rclla, Oaplx!

- [x] Vigener Cipher (16th century, Rome)
      It is an cipher that was being developed by an Roman by named Vigener, In this cipher we have a key(K) and a message (M) when encrypting
      we replicate key length to the length of the message and add then together with mod 26. Similarly, you do the same with the decryption only
      in this case you _substract_ with mod 26
  > Ex:
      Key = CRYPTO
      Message = WHATANICEDAYTODAY
  _Implementation_
     <pre>
     key       =  CRYPTOCRYPTOCRYPT
     message   =  WHATANICEDAYTODAY 
     -------------------------------- + mod 26
     encrypt   =  ZZZJUCLUDTUNWGCQS
     -------------------------------- 
     encrypt   =  ZZZJUCLUDTUNWGCQS
     key       =  CRYPTOCRYPTOCRYPT
     -------------------------------- - mod 26
     message   =  WHATANICEDAYTODAY
     </pre>
- [ ] Roter machines cipher (1870-1943)
      It consists of a key with which a letter mapping was done, Initially the roter was mapped with the key but with each keystroke,
      the key map also routed making it slightly difficult, In cases when an user types `c` for three times the encryption will yeld 3
      different characters thus making it slightly harder to crack, These improvised and then they become multi-roter the infamous
      _`Enigma`_ is an example for Router machine cipher.

Please feel free to open an issue if you would like to implement a new cipher. and can submit a PR.
