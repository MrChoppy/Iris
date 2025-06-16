<script lang="ts">
    import { onMount, onDestroy } from 'svelte';
    import { writable } from 'svelte/store';
    import {isRequestActive} from "$lib/stores/request_status";

    export const transcript = writable('');
    export const isListening = writable(false);
    let typedText = '';

    let recognition: SpeechRecognition | undefined;

    async function sendTranscript(text: string) {
        try {
            isRequestActive.set(true);
            const res = await fetch('http://localhost:8000/api/parse-command', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ text })
            });
            const data = await res.json();
            isRequestActive.set(false);
            console.log('Parsed command response:', data);
            // You can update UI or stores based on `data` here if you want
        } catch (e) {
            isRequestActive.set(false);
            console.error('Failed to send transcript:', e);
        }
    }

    onMount(() => {
        const SpeechRecognitionConstructor =
            window.SpeechRecognition || window.webkitSpeechRecognition;

        if (!SpeechRecognitionConstructor) {
            console.error('Speech recognition not supported.');
            return;
        }

        recognition = new SpeechRecognitionConstructor();
        recognition.lang = 'en-US';
        recognition.continuous = true;
        recognition.interimResults = true;

        recognition.onresult = (event: SpeechRecognitionEvent) => {
            let finalTranscript = '';
            for (let i = event.resultIndex; i < event.results.length; ++i) {
                finalTranscript += event.results[i][0].transcript;
            }
            transcript.set(finalTranscript.trim());
        };

        recognition.onend = () => {
            isListening.set(false);
        };

        recognition.onerror = (event: SpeechRecognitionErrorEvent) => {
            console.error('Speech recognition error:', event.error);
        };
    });

    onDestroy(() => {
        recognition?.stop();
    });

    function start() {
        if (recognition) {
            recognition.start();
            isListening.set(true);
        }
    }

    async function stop() {
        if (recognition) {
            recognition.stop();
            isListening.set(false);
            let currentTranscript = '';
            transcript.subscribe(value => currentTranscript = value)(); // immediate unsubscribe
            if (currentTranscript && currentTranscript.trim().length > 0) {
                await sendTranscript(currentTranscript.trim());
            }
        }
    }

    async function sendTyped() {
        if (typedText.trim().length > 0) {
            await sendTranscript(typedText.trim());
            typedText = ''; // clear after sending
        }
    }
</script>

<style>
    .speech-box {
        padding: 1rem;
        color: white;
    }

    button {
        margin-top: 0.5rem;
        background: #2a2a2a;
        color: white;
        border: none;
        padding: 0.4rem 0.8rem;
        border-radius: 5px;
        cursor: pointer;
    }

    input[type="text"] {
        width: 100%;
        padding: 0.5rem;
        margin-top: 0.5rem;
        border-radius: 5px;
        border: 1px solid #444;
        background: #222;
        color: white;
    }
</style>

<div class="speech-box">
    <h4>Speech Recognizer</h4>
    <p>Transcript: {$transcript}</p>
    <button on:click={start} disabled={$isListening}>Start</button>
    <button on:click={stop} disabled={!$isListening}>Stop</button>

    <div>
        <label for="typedInput">Or type your command:</label>
        <input
                id="typedInput"
                type="text"
                bind:value={typedText}
                placeholder="Type your command here..."
                on:keydown={(e) => { if (e.key === 'Enter') sendTyped(); }}
        />
        <button on:click={sendTyped} disabled={typedText.trim().length === 0}>Send Text</button>
    </div>
</div>
