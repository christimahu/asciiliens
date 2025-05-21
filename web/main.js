// asciiliens/main.js

/*
Documentation for JavaScript (`main.js`):

This script handles the dynamic "Copy Code" button functionality for the code block on the webpage.

1.  **`DOMContentLoaded` Event Listener:**
    * `document.addEventListener('DOMContentLoaded', () => { ... });`
    * This ensures that the JavaScript code runs only after the entire HTML document has been fully loaded and parsed. This is crucial because the script needs to interact with HTML elements (like `pre` and `code` tags), and if it tries to access them before they exist in the DOM, it will result in errors.

2.  **Copy Code Button Logic:**
    * `const codeBlock = document.querySelector('pre code');`
        * Selects the first `<code>` element that is a descendant of a `<pre>` element. This targets the code block containing the "Building and Running" instructions.
    * `if (codeBlock) { ... }`
        * Checks if the `codeBlock` element was successfully found. This prevents errors if the HTML structure changes or the element is not present.
    * `const copyButton = document.createElement('button');`
        * Dynamically creates a new HTML `<button>` element in memory.
    * `copyButton.textContent = 'Copy Code';`
        * Sets the visible text of the button.
    * `copyButton.classList.add('copy-button');`
        * Assigns a CSS class named `copy-button` to the new button. This allows the styling for the button to be defined externally in `styles.css`, keeping presentation separate from behavior.
    * `copyButton.addEventListener('click', () => { ... });`
        * Attaches an event listener to the `copyButton`. When the button is clicked, the function provided as the second argument will execute.
    * `navigator.clipboard.writeText(codeBlock.textContent)`:
        * This is a modern Web API (`Clipboard API`) used for programmatically writing text to the user's clipboard. It asynchronously writes the `textContent` (the plain text within the `<code>` block) to the clipboard.
        * It returns a `Promise`, which allows for handling success (`.then()`) or failure (`.catch()`) of the clipboard operation.
    * `.then(() => { ... })`:
        * If the `writeText` operation is successful, this block is executed.
        * `copyButton.textContent = 'Copied!';`: Provides immediate visual feedback to the user.
        * `setTimeout(() => { ... }, 2000);`: Sets a timer to revert the button text back to "Copy Code" after 2 seconds, indicating that the action is complete.
    * `.catch(err => { ... })`:
        * If the `writeText` operation fails (e.g., due to security restrictions, browser limitations, or user denial), this block is executed.
        * `console.error('Failed to copy text: ', err);`: Logs the error to the browser's developer console for debugging.
        * `copyButton.textContent = 'Error!';`: Informs the user that the copy failed.
    * `codeBlock.parentNode.insertBefore(copyButton, codeBlock);`
        * This DOM manipulation method inserts the `copyButton` into the HTML document.
        * `codeBlock.parentNode` refers to the parent element of the `codeBlock` (which is the `<pre>` tag).
        * `insertBefore(newElement, referenceElement)` inserts `newElement` immediately before `referenceElement` as a child of their common parent. This places the "Copy Code" button just above the code block visually.
*/

document.addEventListener('DOMContentLoaded', () => {
    console.log("ASCIIliens website loaded!");

    // Copy Code Button functionality
    const codeBlock = document.querySelector('pre code');
    if (codeBlock) {
        const copyButton = document.createElement('button');
        copyButton.textContent = 'Copy Code';
        copyButton.classList.add('copy-button'); // Add a class for CSS styling

        copyButton.addEventListener('click', () => {
            navigator.clipboard.writeText(codeBlock.textContent).then(() => {
                copyButton.textContent = 'Copied!';
                setTimeout(() => {
                    copyButton.textContent = 'Copy Code';
                }, 2000);
            }).catch(err => {
                console.error('Failed to copy text: ', err);
                copyButton.textContent = 'Error!';
            });
        });
        // Insert the button before the pre tag
        codeBlock.parentNode.insertBefore(copyButton, codeBlock);
    }
});
