import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import './style.css';

const dualsenseStatus = document.getElementById('dualsense-status') as HTMLElement;
const startBtn = document.getElementById('start-btn') as HTMLButtonElement;
const saveSettingsBtn = document.getElementById('save-settings') as HTMLButtonElement;
const resetSettingsBtn = document.getElementById('reset-settings') as HTMLButtonElement;
const languageSelect = document.getElementById('language-select') as HTMLSelectElement;
const forzaPathInput = document.getElementById('forza-path-input') as HTMLInputElement;
const selectForzaBtn = document.getElementById('select-forza-btn') as HTMLButtonElement;

let backendRunning = false;
let currentLanguage = 'en';
let forzaPath = localStorage.getItem('forzaPath') || '';

// Localization
const translations: Record<string, Record<string, string>> = {
    en: {
        title: 'Forza DualSense Controller',
        subtitle: 'Adaptive Triggers for Forza Horizon',
        status: 'Status',
        'dualsense-disconnected': 'Disconnected',
        'dualsense-connected': 'Connected',
        'game-not-detected': 'Not Detected',
        'game-detected': 'Detected',
        'udp-not-listening': 'Not Listening',
        'udp-listening': 'Listening',
        controls: 'Controls',
        start: 'Start',
        stop: 'Stop',
        settings: 'Settings',
        language: 'Language',
        'select-language': 'Select Language',
        'udp-settings': 'UDP Settings',
        'udp-host': 'UDP Host',
        'udp-port': 'UDP Port',
        'brake-settings': 'Brake Settings',
        'enable-resistance': 'Enable Resistance',
        deadzone: 'Deadzone',
        'baseline-force': 'Baseline Force',
        'max-force': 'Max Force',
        'brake-deadzone': 'Brake Deadzone',
        'brake-baseline-force': 'Brake Baseline Force',
        'brake-max-force': 'Brake Max Force',
        'enable-brake-resistance': 'Enable Brake Resistance',
        'enable-throttle-resistance': 'Enable Throttle Resistance',
        'throttle-settings': 'Throttle Settings',
        'abs-settings': 'ABS Settings',
        'enable-abs': 'Enable ABS',
        'brake-threshold': 'Brake Threshold',
        'startup-settings': 'Startup Settings',
        'enable-startup-pulse': 'Enable Startup Pulse',
        'startup-pulse-force': 'Startup Pulse Force',
        save: 'Save',
        reset: 'Reset',
        cancel: 'Cancel',
        ready: 'Ready to start...',
        starting: 'Starting Python backend...',
        started: 'Backend started successfully',
        stopping: 'Stopping Python backend...',
        stopped: 'Backend stopped successfully',
        'settings-saved': 'Settings saved',
        'error-starting': 'Error starting backend',
        'error-stopping': 'Error stopping backend',
        'error-saving': 'Error saving settings',
        'forza-path': 'Forza Horizon 6 Path',
        'dualsense': 'DualSense',
        'game': 'Game',
        'udp': 'UDP',
        'github-repo': 'GitHub Repository',
    },
    ru: {
        title: 'Контроллер Forza DualSense',
        subtitle: 'Адаптивные курки для Forza Horizon',
        status: 'Статус',
        'dualsense-disconnected': 'Отключен',
        'dualsense-connected': 'Подключен',
        'game-not-detected': 'Не обнаружена',
        'game-detected': 'Обнаружена',
        'udp-not-listening': 'Не слушает',
        'udp-listening': 'Слушает',
        controls: 'Управление',
        start: 'Запуск',
        stop: 'Стоп',
        settings: 'Настройки',
        language: 'Язык',
        'select-language': 'Выберите язык',
        'udp-settings': 'Настройки UDP',
        'udp-host': 'UDP хост',
        'udp-port': 'UDP порт',
        'brake-settings': 'Настройки тормозов',
        'enable-resistance': 'Включить сопротивление',
        deadzone: 'Мертвая зона',
        'baseline-force': 'Базовая сила',
        'max-force': 'Макс. сила',
        'brake-deadzone': 'Мертвая зона тормозов',
        'brake-baseline-force': 'Базовая сила тормозов',
        'brake-max-force': 'Макс. сила тормозов',
        'enable-brake-resistance': 'Включить сопротивление тормозов',
        'enable-throttle-resistance': 'Включить сопротивление газа',
        'throttle-settings': 'Настройки газа',
        'abs-settings': 'Настройки ABS',
        'enable-abs': 'Включить ABS',
        'brake-threshold': 'Порог тормозов',
        'startup-settings': 'Настройки запуска',
        'enable-startup-pulse': 'Включить пульс при запуске',
        'startup-pulse-force': 'Сила пульса при запуске',
        save: 'Сохранить',
        reset: 'Сброс',
        cancel: 'Отмена',
        ready: 'Готов к запуску...',
        starting: 'Запуск Python бэкенда...',
        started: 'Бэкенд успешно запущен',
        stopping: 'Остановка Python бэкенда...',
        stopped: 'Бэкенд успешно остановлен',
        'settings-saved': 'Настройки сохранены',
        'error-starting': 'Ошибка запуска бэкенда',
        'error-stopping': 'Ошибка остановки бэкенда',
        'error-saving': 'Ошибка сохранения настроек',
        'forza-path': 'Путь к Forza Horizon 6',
        'dualsense': 'DualSense',
        'game': 'Игра',
        'udp': 'UDP',
        'github-repo': 'Гитхаб репозиторий',
    }
};

function setLanguage(lang: string) {
    currentLanguage = lang;
    localStorage.setItem('language', lang);
    
    document.querySelectorAll('[data-i18n]').forEach(el => {
        const key = el.getAttribute('data-i18n');
        if (key && translations[lang][key]) {
            // Check if element has span children (status-bar-item)
            const span = el.querySelector('span:last-child');
            if (span) {
                span.textContent = translations[lang][key];
            } else {
                el.textContent = translations[lang][key];
            }
        }
    });
}

function loadLanguage() {
    const savedLang = localStorage.getItem('language') || 'en';
    languageSelect.value = savedLang;
    setLanguage(savedLang);
}

languageSelect.addEventListener('change', (e) => {
    const target = e.target as HTMLSelectElement;
    setLanguage(target.value);
});

// Titlebar controls
// Wait for DOM to be ready
document.addEventListener('DOMContentLoaded', () => {
    const minimizeBtn = document.getElementById('minimize-btn') as HTMLButtonElement;
    const maximizeBtn = document.getElementById('maximize-btn') as HTMLButtonElement;
    const closeBtn = document.getElementById('close-btn') as HTMLButtonElement;

    console.log('DOM loaded, buttons found:', minimizeBtn, maximizeBtn, closeBtn);

    if (minimizeBtn) {
        minimizeBtn.addEventListener('click', (e) => {
            e.preventDefault();
            e.stopPropagation();
            console.log('Minimize button clicked');
            invoke('minimize_window').catch(err => console.error('Error minimizing window:', err));
        });
    }

    if (maximizeBtn) {
        maximizeBtn.addEventListener('click', (e) => {
            e.preventDefault();
            e.stopPropagation();
            console.log('Maximize button clicked');
            invoke('maximize_window').catch(err => console.error('Error maximizing window:', err));
        });
    }

    if (closeBtn) {
        closeBtn.addEventListener('click', (e) => {
            e.preventDefault();
            e.stopPropagation();
            console.log('Close button clicked');
            invoke('close_window').catch(err => console.error('Error closing window:', err));
        });
    }
});

// Slider value updates
const sliders = [
    { input: document.getElementById('brake-deadzone') as HTMLInputElement, value: document.getElementById('brake-deadzone-value') as HTMLInputElement },
    { input: document.getElementById('brake-baseline-force') as HTMLInputElement, value: document.getElementById('brake-baseline-force-value') as HTMLInputElement },
    { input: document.getElementById('brake-max-force') as HTMLInputElement, value: document.getElementById('brake-max-force-value') as HTMLInputElement },
];

sliders.forEach(slider => {
    if (slider.input && slider.value) {
        // Update input when slider changes
        slider.input.addEventListener('input', () => {
            slider.value.value = slider.input.value;
        });
        
        // Update slider when input changes
        slider.value.addEventListener('input', () => {
            let newValue = parseInt(slider.value.value);
            const min = parseInt(slider.input.min);
            const max = parseInt(slider.input.max);
            
            // Clamp value to min/max
            if (isNaN(newValue)) newValue = min;
            if (newValue < min) newValue = min;
            if (newValue > max) newValue = max;
            
            slider.input.value = newValue.toString();
            slider.value.value = newValue.toString();
        });
    }
});

async function startBackend() {
    try {
        await invoke<string>('start_python_backend');
        backendRunning = true;
        dualsenseStatus.className = 'status-bar-item connected';
        const statusSpan = dualsenseStatus.querySelector('span:last-child');
        if (statusSpan) statusSpan.textContent = 'Connected';
        startBtn.textContent = translations[currentLanguage]['stop'];
        startBtn.dataset.i18n = 'stop';
        
        // Send start command to Python
        await sendCommandToPython({ type: 'start' });
        
        // Launch Forza if path is set
        if (forzaPath) {
            try {
                await invoke('launch_exe', { path: forzaPath });
            } catch (error) {
                console.error('Error launching Forza:', error);
            }
        }
    } catch (error) {
        console.error('Error starting backend:', error);
    }
}

async function stopBackend() {
    try {
        await invoke<string>('stop_python_backend');
        backendRunning = false;
        dualsenseStatus.className = 'status-bar-item disconnected';
        const statusSpan = dualsenseStatus.querySelector('span:last-child');
        if (statusSpan) statusSpan.textContent = 'Disconnected';
        startBtn.textContent = translations[currentLanguage]['start'];
        startBtn.dataset.i18n = 'start';
    } catch (error) {
        console.error('Error stopping backend:', error);
    }
}

async function sendCommandToPython(command: any) {
    try {
        const commandStr = JSON.stringify(command);
        await invoke<string>('send_command_to_python', { command: commandStr });
    } catch (error) {
        console.error(`Error sending command: ${error}`);
    }
}

async function getStatus() {
    if (backendRunning) {
        await sendCommandToPython({ type: 'get_status' });
    }
}

startBtn.addEventListener('click', () => {
    if (backendRunning) {
        stopBackend();
    } else {
        startBackend();
    }
});

// Save Forza path on input change
forzaPathInput.addEventListener('input', () => {
    forzaPath = forzaPathInput.value;
    localStorage.setItem('forzaPath', forzaPath);
});

// Browse button for Forza path
selectForzaBtn.addEventListener('click', async () => {
    try {
        const path = await invoke<string>('open_file_dialog');
        if (path) {
            forzaPath = path;
            forzaPathInput.value = path;
            localStorage.setItem('forzaPath', path);
        }
    } catch (error) {
        console.error('Error opening file dialog:', error);
    }
});

// Update Forza path input on load
if (forzaPath) {
    forzaPathInput.value = forzaPath;
}

saveSettingsBtn.addEventListener('click', async () => {
    try {
        const settings = {
            udp_host: '127.0.0.1',
            udp_port: parseInt((document.getElementById('udp-port') as HTMLInputElement).value),
            brake_deadzone: parseInt((document.getElementById('brake-deadzone') as HTMLInputElement).value),
            brake_baseline_force: parseInt((document.getElementById('brake-baseline-force') as HTMLInputElement).value),
            brake_max_force: parseInt((document.getElementById('brake-max-force') as HTMLInputElement).value),
            enable_brake_resistance: (document.getElementById('enable-brake-resistance') as HTMLInputElement).checked,
            enable_throttle_resistance: (document.getElementById('enable-throttle-resistance') as HTMLInputElement).checked,
            enable_abs: (document.getElementById('enable-abs') as HTMLInputElement).checked,
            enable_startup_pulse: (document.getElementById('enable-startup-pulse') as HTMLInputElement).checked,
        };
        
        await invoke('save_settings', { settings });
        console.log('Settings saved');
        
        // Try to send settings to Python backend first
        if (backendRunning) {
            console.log('Sending settings to Python backend...');
            try {
                await sendCommandToPython({ type: 'update_settings', settings });
                console.log('Settings sent to Python backend');
            } catch (error) {
                console.error('Error sending settings to Python:', error);
            }
        }
        
        // Force restart backend to apply settings
        console.log('Force restarting backend...');
        
        try {
            await stopBackend();
            console.log('Backend stopped');
        } catch (error) {
            console.error('Error stopping backend:', error);
        }
        
        // Delay to ensure backend is fully stopped
        await new Promise(resolve => setTimeout(resolve, 1000));
        
        try {
            await startBackend();
            console.log('Backend started');
        } catch (error) {
            console.error('Error starting backend:', error);
        }
    } catch (error) {
        console.error('Error saving settings:', error);
    }
});

resetSettingsBtn.addEventListener('click', () => {
    (document.getElementById('udp-port') as HTMLInputElement).value = '5300';
    (document.getElementById('brake-deadzone') as HTMLInputElement).value = '50';
    (document.getElementById('brake-baseline-force') as HTMLInputElement).value = '15';
    (document.getElementById('brake-max-force') as HTMLInputElement).value = '60';
    (document.getElementById('enable-brake-resistance') as HTMLInputElement).checked = true;
    (document.getElementById('enable-throttle-resistance') as HTMLInputElement).checked = true;
    (document.getElementById('enable-abs') as HTMLInputElement).checked = true;
    (document.getElementById('enable-startup-pulse') as HTMLInputElement).checked = true;
    
    // Update slider values display
    (document.getElementById('brake-deadzone-value') as HTMLInputElement).value = '50';
    (document.getElementById('brake-baseline-force-value') as HTMLInputElement).value = '15';
    (document.getElementById('brake-max-force-value') as HTMLInputElement).value = '60';
});

// Listen for Python backend responses
listen('python-response', (event) => {
    try {
        const response = JSON.parse(event.payload as string);
        if (response.status === 'started') {
            dualsenseStatus.className = 'status-bar-item connected';
            const statusSpan = dualsenseStatus.querySelector('span:last-child');
            if (statusSpan) statusSpan.textContent = 'Connected';
        } else if (response.status === 'stopped') {
            dualsenseStatus.className = 'status-bar-item disconnected';
            const statusSpan = dualsenseStatus.querySelector('span:last-child');
            if (statusSpan) statusSpan.textContent = 'Disconnected';
        } else if (response.type === 'status') {
            console.log(`Status: DualSense=${response.dualsense_connected}, Running=${response.running}`);
        } else if (response.error) {
            console.error(`Error: ${response.error}`);
        }
    } catch (error) {
        console.error(`Response: ${event.payload}`);
    }
});

// Poll status every 5 seconds
setInterval(getStatus, 5000);

// Load saved language
loadLanguage();

// GitHub link click handler
const githubLink = document.getElementById('github-link') as HTMLAnchorElement;
if (githubLink) {
    githubLink.addEventListener('click', async (e) => {
        e.preventDefault();
        const url = githubLink.getAttribute('href');
        if (url) {
            try {
                await invoke('open_url', { url });
            } catch (error) {
                console.error('Error opening URL:', error);
            }
        }
    });
}

// Auto-start backend on load
startBackend();
