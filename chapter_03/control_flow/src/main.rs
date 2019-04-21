fn main() {
   let num = 3;
   
   /* good ol' if-else */
   if num < 5 {
       println!("num is less than 5");
   } else {
       println!("num is not less than 5");
   }

   /* The condition must always be 'bool' otherwise rust will throw error */
   /*if num {
       println!("num is 3");
   }*/
   /* shadowing of 'num' variable */
   let num = 7;

   if num % 4 == 0 {
       println!("num is divisible by 4");
   } else if num % 3 == 0 {
       println!("num is divisible by 3");
   } else if num % 2 == 0 {
       println!("num is divisible by 2");
   } else {
       println!("num is not divisible by 4,3,2");
   }

   /* We can use if on the right side of the if statement
    * Meaning a variable is assigned a value if the condition holds*/

   let condition = true;
   let number = if condition {
       5
   } else {
       6
   };

   println!("The value of number is: {}", number);
   
}
