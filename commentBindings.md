- Pourquoi ne pas transformer les enum C en véritables enum Rust ?

- Les types explicites peuvent être très utiles, notamment pour la sécurité et la clarté. Et personement j'utilise `IntoPrimitive` et `FromPrimitive` qui on besion de cette anotation de type sur les enum `#[repr(T)]`

- Si tu ne précises pas le type, c’est certes plus simple à générer, mais tu perds en précision.

- Il y a un problème fondamental : certains enum dans les fichiers .h n’ont pas de type explicite, et leur type est déduit implicitement en fonction des valeurs. Cela peut poser des problèmes lors de la conversion vers Rust si tu shouaite garader l'annotation de type. 
  
- Dans tout les cas la version actuel pose un probleme car du dis que les valeur de l'enum (quil sois dans un enum ou dans une const) est de type c_uint qui est un type qui depend de l'archi de la machine alors que le type defini par le C depend des valeur. il y a un cas de figure ou cela ne va pas marche: si les plus grande valeur de l'enum ne rentre pas dans un entier machine (int ou uint) exemple si 
  ```
  pub type EdsImageSize = ::std::os::raw::c_uint;
  pub const Unknown: EdsImageSize = 4294967295;
  ``` 
  avec une achi 16bits. Cela ne va pas marche car `4294967295` ne rendre pas dans un u16.
  Si l'on veux garder l'annoation de type (ce que je recommande) ce n'est pas simple car il faut annaliser l'enum pour voir quel est le type le plus petit dans le quel la plus grande valaur de l'enum rentre. Dans notre exemple.
  il faudrai remplacer `pub type EdsImageSize = ::std::os::raw::c_uint;` par 
  `pub type EdsImageSize = u32;` pour etre sur que `4294967295` puisse etre stoquer quel que sois l'archi sur la quel le code est exec. 

  - peut etre quil est posible de le faire en executent un bou de C avec size_of pour deduire quel type dans rust utiliser ce qui eviterai de faire de l'analise de code ? 




```C
typedef enum
{
    AEB = 0x01,
    ISOB = 0x02,
    WBB = 0x04,
    FEB = 0x08,
    Unknown = 0xffffffff,

} EdsBracket;
```
Et converti en 
```rust
pub const EdsBracket_AEB: EdsBracket = 1;
pub const EdsBracket_ISOB: EdsBracket = 2;
pub const EdsBracket_WBB: EdsBracket = 4;
pub const EdsBracket_FEB: EdsBracket = 8;
pub const EdsBracket_Unknown: EdsBracket = 4294967295;
pub type EdsBracket = ::std::os::raw::c_uint;
```

Dans un archi 16bits il y a un probleme. 

je proposer au minimume de changer `pub type EdsBracket = ::std::os::raw::c_uint;` en `pub type EdsBracket = u32;`

et dans le meuilleur des cas transfmer en 

```rust
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum EdsBracket {
    AEB = 1,
    ISOB = 2,
    WBB = 4,
    FEB = 8,
    Unknown = 4294967295,
}
```

la difficulter reside dans le fais de trouver ce `u32` qui est le plus petit type dans le quel rendre `4294967295`

sinon une version que je juge temporaire serai de ne pas donner le type dans lenum et decrire 

```rust
#[derive(Debug, Clone, Copy)]
pub enum EdsBracket {
    AEB = 1,
    ISOB = 2,
    WBB = 4,
    FEB = 8,
    Unknown = 4294967295,
}
```

Mais le probleme rest le meme car dans le cas des `#define` le type n'est pas expliciter alors que les `const` rust doivent avoir un type explicite

