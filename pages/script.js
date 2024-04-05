import llvl, { run } from "./llvl.js";

await llvl();

window.button_click = () => {
    let c = document.getElementById("code_area");
    let result = run(c.value);
    console.log(result);

    document.getElementById("result_area").innerHTML = result;
}
