// Connect to WebSocket server
const ws = new WebSocket('ws://127.0.0.1:8081/chat');

// Helper function to update the response text
function updateResponse(text) {
    const responseElement = document.getElementById('response');
    if (responseElement) {
        responseElement.textContent = text;
    }
}

// WebSocket event handlers
ws.onopen = () => updateResponse("Connected to WebSocket server!");
ws.onmessage = (event) => updateResponse("Received: " + event.data);
ws.onerror = (error) => updateResponse("WebSocket Error: " + error);
ws.onclose = () => updateResponse("Disconnected from WebSocket server!");

// Send message to WebSocket server
function sendMessage() {
    const messageInput = document.getElementById('message-input');
    const message = messageInput ? messageInput.value : '';
    if (message) {
        console.log('Sending: ' + message);
        ws.send(message);
    }
}

// Attach event listener to send button
document.getElementById('send-button')?.addEventListener('click', sendMessage);

