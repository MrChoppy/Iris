import {isRequestActive} from "../stores/request_status";
import {chatResponse} from "../stores/chat_response";

export async function sendMessage(formattedPrompt: string): Promise<void> {
    if (formattedPrompt.trim().length === 0) return;
    try {
        const response = await fetch("/api/chat/send", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ text: formattedPrompt }),
        });

        if (!response.ok) {
            console.error("Failed to send message:", response.statusText);
            return;
        }

        const result = await response.json();
        chatResponse.set(result.reply);
        console.log("Message sent:", result);
    } catch (error) {
        console.error("Error sending message:", error);
    } finally {
        isRequestActive.set(false);
    }
}

export async function handleUserInput(userText: string) {
    isRequestActive.set(true);

    try {
        const response = await fetch("/api/chat/command-search", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ userText }),
        });

        if (!response.ok) {
            console.error("Failed to get commands:", response.statusText);
            return;
        }

        const { prompt } = await response.json();
        console.log("Prompt for assistant:", prompt);

        await sendMessage(prompt);
    } catch (error) {
        console.error("Error in handleUserInput:", error);
    }
}
