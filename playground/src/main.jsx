import React, { useState, useCallback, useEffect } from 'react';
import useWebSocket, { ReadyState } from 'react-use-websocket';
import ReactDOM from 'react-dom/client'

const WS_URL = 'ws://localhost:8080/ws';

export const WSConnection = () => {
    const [messageHistory, setMessageHistory] = useState([]);
    const { sendMessage, lastMessage, readyState } = useWebSocket(WS_URL);

    useEffect(() => {
        if (lastMessage !== null) {
            setMessageHistory(prev => prev.concat(lastMessage));
        }
    }, [lastMessage])

    const handleClickSendMessage = useCallback(() => sendMessage('ping'), []);

    const connectionStatus = {
        [ReadyState.CONNECTING]: 'Connecting',
        [ReadyState.OPEN]: 'Open',
        [ReadyState.CLOSING]: 'Closing',
        [ReadyState.CLOSED]: 'Closed',
        [ReadyState.UNINSTANTIATED]: 'Uninstantiated',
    }[readyState];

    return (
        <>
            <div
                style={{
                    fontFamily: 'system-ui, sans-serif',
                    display: 'flex',
                    flexDirection: 'column',
                    alignItems: 'center',
                    minHeight: '100vh',
                    background: '#f5f7fa',
                    color: '#333',
                    padding: '2rem',
                }}
            >
                <div
                    style={{
                        background: 'white',
                        borderRadius: '12px',
                        boxShadow: '0 4px 10px rgba(0,0,0,0.1)',
                        padding: '2rem',
                        maxWidth: '480px',
                        width: '100%',
                        textAlign: 'center',
                    }}
                >
                    <h2 style={{ marginBottom: '1rem', color: '#2d3748' }}>
                        WebSocket Message Viewer
                    </h2>

                    <button
                        onClick={handleClickSendMessage}
                        disabled={readyState !== ReadyState.OPEN}
                        style={{
                            backgroundColor:
                                readyState === ReadyState.OPEN ? '#4f46e5' : '#9ca3af',
                            color: 'white',
                            border: 'none',
                            borderRadius: '8px',
                            padding: '0.6rem 1.2rem',
                            cursor:
                                readyState === ReadyState.OPEN ? 'pointer' : 'not-allowed',
                            fontSize: '1rem',
                            transition: 'background 0.2s',
                        }}
                    >
                        Ping
                    </button>

                    <p style={{ marginTop: '1rem', fontSize: '0.95rem', color: '#555' }}>
                        WebSocket is currently{' '}
                        <strong
                            style={{
                                color:
                                    connectionStatus === 'Open'
                                        ? '#16a34a'
                                        : connectionStatus === 'Closed'
                                            ? '#dc2626'
                                            : '#d97706',
                            }}
                        >
                            {connectionStatus}
                        </strong>
                    </p>

                    {lastMessage && (
                        <div
                            style={{
                                marginTop: '1rem',
                                backgroundColor: '#f1f5f9',
                                borderRadius: '8px',
                                padding: '0.75rem',
                                fontSize: '0.9rem',
                                color: '#1e293b',
                            }}
                        >
                            <strong>Last message:</strong> {lastMessage.data}
                        </div>
                    )}

                    <ul
                        style={{
                            listStyle: 'none',
                            padding: 0,
                            marginTop: '1rem',
                            maxHeight: '180px',
                            overflowY: 'auto',
                            backgroundColor: '#f8fafc',
                            borderRadius: '8px',
                            border: '1px solid #e2e8f0',
                        }}
                    >
                        {messageHistory.map((message, idx) => (
                            <li
                                key={idx}
                                style={{
                                    padding: '0.5rem 0.75rem',
                                    borderBottom:
                                        idx < messageHistory.length - 1
                                            ? '1px solid #e2e8f0'
                                            : 'none',
                                    fontSize: '0.9rem',
                                    textAlign: 'left',
                                    color: '#334155',
                                }}
                            >
                                {message ? message.data : ''}
                            </li>
                        ))}
                    </ul>
                </div>
            </div>
        </>
    )
}

ReactDOM.createRoot(document.getElementById('root')).render(<WSConnection />)
