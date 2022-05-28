Stuff covered
1. structs can be created in three ways 
   ```rs
   // Way 1  
   struct User {
       username: String,
       active: bool,
   };
   // Way 2
   struct Color(u32, u32, u32);

   //way 3
   struct AlwaysEqual;
   ```

2. Structs are owned by the instances, hence we used String instead of &str for username. More on this in chapter 10