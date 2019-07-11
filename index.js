function fibonacci(number) {
    if (number < 2) {
        return number;
    }
    else
        return (fibonacci(number - 1) +
            fibonacci(number - 2));
}

let number = 46;
for (let counter = 0; counter < number; counter++) {
    console.log(fibonacci(counter));
}
