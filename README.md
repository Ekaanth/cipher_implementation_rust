# cipher_implementations_rust

Project contains the rust implementation's of all the cryptography methods I learn.
List of ciphers I will be implementing are:

- [ ] Playfair
- [x] Hill Cipher
- [x] Caesar Cipher
- [x] Keyword Cipher
- [x] Affine Cipher
- [x] Vigener Cipher (16th century, Rome)
      It is an cipher that was being developed by an Roman by named Vigener, IN this cipher we have a key(K) and a message (M) when encrypting
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
