const usernameInput = document.getElementById("username");
const ageInput = document.getElementById("age");
const passwordInput = document.getElementById("password");
const confirmPasswordInput = document.getElementById("confirmPassword");
const submitBtnInput = document.getElementById("submitBtn");
const clearInput = document.getElementById("clear");

const usernametext = document.getElementById("usernametext");
const agetext = document.getElementById("agetext");
const passwordtext = document.getElementById("passwordtext");
const confirmpasswordtext = document.getElementById("confirmpasswordtext");

usernameRegex=/^[A-Za-z][A-Za-z0-9_]{5,20}$/;
ageRegex=/^(1[89]|[2-9]\d)$/;
passwordRegex=/^[A-Za-z\d!@#$%^&*()_\-+]{8,}$/;

function clearList() {
    document.getElementById("registrationForm").reset();
}

function submit() {
    const username=usernameInput.value;
    const age = ageInput.value;
    const password=passwordInput.value;
    const confirmPassword=confirmPasswordInput.value;
    let x=0;

    if(!usernameRegex.test(username)){
        usernametext.style.visibility = "visible";
        x=1;
    }
    else {usernametext.style.visibility = "hidden";}

    if (!ageRegex.test(age)){
        agetext.style.visibility = "visible";
        x=1;
    }
    else {agetext.style.visibility = "hidden";}

    if(!passwordRegex.test(password)){
        passwordtext.style.visibility = "visible";
        x=1;
    }
    else {agetext.style.visibility = "hidden";}

    if(confirmPassword!=password){
        confirmpasswordtext.style.visibility = "visible";
        x=1;
    }
    else {confirmpasswordtext.style.visibility = "hidden";}
    
    if(x===1) {return;}
    else{alert("Tot corect");}

}

clearInput.addEventListener("click", clearList);
submitBtnInput.addEventListener("click", submit);