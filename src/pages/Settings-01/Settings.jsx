
import { createEffect, createSignal, onMount } from 'solid-js';
import './Settings.css'
import GlobalSettingsPage from './Settings-Categories/Global/GlobalSettingsPage';
import RealDebridSettings from './Settings-Categories/RealDebrid/RealDebrid';

function Settings() {
    const [activeCategory, setActiveCategory] = createSignal('global-display');
    const [activeGroup, setActiveGroup] = createSignal('global')

    return (
        <div className="settings content-page">
            <SettingsSidebar setActiveCategory={setActiveCategory} setActiveGroup={setActiveGroup} />
            <div className="settings-content">
                {activeGroup() === 'global' ? (
                    <GlobalSettingsPage settingsPart={activeCategory()} />
                ) : (
                    <RealDebridSettings />
                )}
            </div>
        </div>
    )
}

function SettingsSidebar({ setActiveCategory, setActiveGroup }) {

    onMount(() => {
        // Activate the first DHT element by default
        handleActivateElem('settings-display', 'global-display');
        setActiveGroup('global');
    });

    // Helper function to reset all backgrounds to transparent
    function changeAllToDefault() {
        const allDivs = document.querySelectorAll('.settings-sidebar-group-list-category a');
        allDivs.forEach((elem) => {
            elem.style.backgroundColor = 'transparent';
        });
    }

    function handleActivateElem(elemID, category) {
        changeAllToDefault();
        const selectedElem = document.getElementById(elemID);
        if (selectedElem) {
            selectedElem.style.backgroundColor = 'var(--secondary-30-selected-color)';
        }

        if (category.startsWith('global')) {
            setActiveGroup('global');
        } else {
            setActiveGroup('realdebrid');
        }
        // Update the active category state
        setActiveCategory(category);
    }

    return (
        <div className="settings-sidebar">
            <div className="settings-sidebar-group">
                <ul className="settings-sidebar-group-list-category">
                    <p className="settings-sidebar-group-title">
                        Global
                    </p>
                    <a id="settings-display" onClick={() => handleActivateElem("settings-display", "global-display")}>
                        <svg width="24" xmlns="http://www.w3.org/2000/svg" height="24" viewBox="-196 798.3 24 24" style="-webkit-print-color-adjust::exact" fill="none"><g class="fills"><rect rx="0" ry="0" x="-196" y="798.3" width="24" height="24" class="frame-background" /></g><g class="frame-children"><path d="M-176 805.3h-9" style="fill:none" class="fills" /><g stroke-linecap="round" stroke-linejoin="round" class="strokes"><path d="M-176 805.3h-9" style="fill:none;fill-opacity:none;stroke-width:2;stroke:var(--text-color);stroke-opacity:1" class="stroke-shape" /></g><path d="M-182 815.3h-9" style="fill:none" class="fills" /><g stroke-linecap="round" stroke-linejoin="round" class="strokes"><path d="M-182 815.3h-9" style="fill:none;fill-opacity:none;stroke-width:2;stroke:var(--text-color);stroke-opacity:1" class="stroke-shape" /></g><circle cx="-179" cy="815.3" style="fill:none" class="fills" r="3" /><g stroke-linecap="round" stroke-linejoin="round" class="strokes"><circle cx="-179" cy="815.3" style="fill:none;fill-opacity:none;stroke-width:2;stroke:var(--text-color);stroke-opacity:1" class="stroke-shape" r="3" /></g><circle cx="-189" cy="805.3" style="fill:none" class="fills" r="3" /><g stroke-linecap="round" stroke-linejoin="round" class="strokes"><circle cx="-189" cy="805.3" style="fill:none;fill-opacity:none;stroke-width:2;stroke:var(--text-color);stroke-opacity:1" class="stroke-shape" r="3" /></g></g></svg>
                        <span>App Settings</span>
                    </a>
                    <a id="settings-dns" onClick={() => handleActivateElem("settings-dns", "global-dns")}>
                        <svg width="24" xmlns="http://www.w3.org/2000/svg" height="24" viewBox="-196 873.3 24 24" style="-webkit-print-color-adjust::exact" fill="none"><g class="fills"><rect rx="0" ry="0" x="-196" y="873.3" width="24" height="24" class="frame-background" /></g><g class="frame-children"><circle cx="-184" cy="885.3" style="fill:none" class="fills" r="10" /><g stroke-linecap="round" stroke-linejoin="round" class="strokes"><circle cx="-184" cy="885.3" style="fill:none;fill-opacity:none;stroke-width:2;stroke:var(--text-color);stroke-opacity:1" class="stroke-shape" r="10" /></g><path d="M-184 875.3c-5.333 5.6-5.333 14.4 0 20 5.333-5.6 5.333-14.4 0-20" style="fill:none" class="fills" /><g stroke-linecap="round" stroke-linejoin="round" class="strokes"><path d="M-184 875.3c-5.333 5.6-5.333 14.4 0 20 5.333-5.6 5.333-14.4 0-20" style="fill:none;fill-opacity:none;stroke-width:2;stroke:var(--text-color);stroke-opacity:1" class="stroke-shape" /></g><path d="M-194 885.3h20" style="fill:none" class="fills" /><g stroke-linecap="round" stroke-linejoin="round" class="strokes"><path d="M-194 885.3h20" style="fill:none;fill-opacity:none;stroke-width:2;stroke:var(--text-color);stroke-opacity:1" class="stroke-shape" /></g></g></svg>
                        <span>DNS Settings</span>
                    </a>
                    <a id="settings-install" onClick={() => handleActivateElem("settings-install", "global-install")}>
                        <svg width="24" xmlns="http://www.w3.org/2000/svg" height="24" viewBox="-196 948.3 24 24" style="-webkit-print-color-adjust::exact" fill="none"><g class="fills"><rect rx="0" ry="0" x="-196" y="948.3" width="24" height="24" class="frame-background" /></g><g class="frame-children"><path d="M-181.3 954.6a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a5.999 5.999 0 0 1-7.94 7.94l-6.91 6.91a2.122 2.122 0 0 1-3-3l6.91-6.91a5.999 5.999 0 0 1 7.94-7.94z" style="fill:none" class="fills" /><g stroke-linecap="round" stroke-linejoin="round" class="strokes"><path d="M-181.3 954.6a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a5.999 5.999 0 0 1-7.94 7.94l-6.91 6.91a2.122 2.122 0 0 1-3-3l6.91-6.91a5.999 5.999 0 0 1 7.94-7.94z" style="fill:none;fill-opacity:none;stroke-width:2;stroke:var(--text-color);stroke-opacity:1" class="stroke-shape" /></g></g></svg>
                        <span>Install Settings</span>
                    </a>
                    <a id="settings-cache" onClick={() => handleActivateElem("settings-cache", "global-cache")}>
                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-database"><ellipse cx="12" cy="5" rx="9" ry="3"/><path d="M3 5v14a9 3 0 0 0 18 0V5"/><path d="M3 12a9 3 0 0 0 18 0"/></svg>
                        <span>Cache & Logs Settings</span>
                    </a>


                </ul>
                <ul className="settings-sidebar-group-list-category">
                    <p className="settings-sidebar-group-title">
                        Downloaders
                    </p>
                    <a id="settings-realdebrid" onClick={() => handleActivateElem("settings-realdebrid", "realdebrid")}>
                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-zap"><polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2"/></svg>
                        <span>Real-Debrid</span>
                    </a>
                </ul>
            </div>
        </div>
    )
}

export default Settings;