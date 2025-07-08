import { createSignal, onMount } from 'solid-js';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import './RealDebrid.css';

function RealDebridSettings() {
    const [authUrl, setAuthUrl] = createSignal('');
    const [userCode, setUserCode] = createSignal('');
    const [isAuthenticating, setIsAuthenticating] = createSignal(false);
    const [authStatus, setAuthStatus] = createSignal(''); // e.g., 'success', 'failure'

    onMount(() => {
        const unlistenAuthPrompt = listen('realdebrid-auth-prompt', (event) => {
            const { verification_url, user_code } = event.payload;
            setAuthUrl(verification_url);
            setUserCode(user_code);
            setIsAuthenticating(true);
            setAuthStatus('');
        });

        const unlistenAuthSuccess = listen('realdebrid-auth-success', () => {
            setIsAuthenticating(false);
            setAuthStatus('success');
        });

        const unlistenAuthFailure = listen('realdebrid-auth-failure', (event) => {
            setIsAuthenticating(false);
            setAuthStatus(`failure: ${event.payload}`);
        });

        return () => {
            unlistenAuthPrompt();
            unlistenAuthSuccess();
            unlistenAuthFailure();
        };
    });

    const handleAuthClick = async () => {
        try {
            await invoke('rd_authenticate');
        } catch (error) {
            console.error('Failed to start Real-Debrid authentication:', error);
            setAuthStatus(`failure: ${error}`);
        }
    };

    return (
        <div class="real-debrid-settings">
            <h2>Real-Debrid Authentication</h2>
            {!isAuthenticating() && (
                <button onClick={handleAuthClick}>Login to Real-Debrid</button>
            )}
            {isAuthenticating() && (
                <div class="auth-prompt">
                    <p>Please go to <a href={authUrl()} target="_blank">{authUrl()}</a></p>
                    <p>And enter the code: <strong>{userCode()}</strong></p>
                </div>
            )}
            {authStatus() === 'success' && (
                <p class="auth-status success">Successfully authenticated with Real-Debrid!</p>
            )}
            {authStatus().startsWith('failure') && (
                <p class="auth-status failure">Authentication failed: {authStatus()}</p>
            )}
        </div>
    );
}

export default RealDebridSettings;