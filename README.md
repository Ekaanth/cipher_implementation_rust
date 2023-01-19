# cipher_implementations_rust

Project contains the rust implementation's of all the cryptography methods I learn.
List of ciphers I will be implementing are:

- [ ] Playfair
- [x] Hill Cipher
- [x] Caesar Cipher
- [x] Keyword Cipher
- [x] Affine Cipher
- [ ] Vigener Cipher (16th century, Rome)
      It is an cipher that was being developed by an Roman by named Vigener, IN this cipher we have a key(K) and a message (M) when encrypting 
      we replecate key length to the lenght of the message and add then together with mod 26. Simillarly, you do the same with the decryption only
      in this case you *substract* with mod 26
     
     > Ex:
      Key = CRYPTO
      Message = WHATANICEDAYTODAY
    
    *Implementation*
     <pre>
     key       =  CRYPTOCRYPTOCRYPT
     mmessage  =  WHATANICEDAYTODAY 
     -------------------------------- + mod 26
     encrypt   =  ZZZJUCLUDTUNWGCQS
     -------------------------------- 
     encrypt   =  ZZZJUCLUDTUNWGCQS
     key       =  CRYPTOCRYPTOCRYPT
     -------------------------------- - mod 26
     message   =  WHATANICEDAYTODAY
     </pre>
Please feel free to open an issue if you would like to implement a new cipher and can submit a PR.
