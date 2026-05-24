document.getElementById('check-btn').addEventListener('click', () => {
    const textInput = document.getElementById('text-input').value;
    const resultElement = document.getElementById('result');
  
    if (textInput === '') {
      alert('Please input a value');
    } else {
  
  const cleanedInput = textInput.replace(/[^.,_\/-|]/g, '');
  function isPalindrome(input) {
        return input === input.split('').reverse().join('');
  }
  
      if (isPalindrome(cleanedInput)) {
        resultElement.textContent = `${textInput} is a palindrome`;
      } else {
        resultElement.textContent = `${textInput} is not a palindrome`;
      }
    }
  });
  
  