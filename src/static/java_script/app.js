let my_btn=document.getElementById("mybtn");
console.log("hello my name is vikas")
my_btn.addEventListener("click",handleItem);
let item=10;
let item2=20;
function handleItem(){
    console.log(`the total cost of item is ${item + item2}`);
}
