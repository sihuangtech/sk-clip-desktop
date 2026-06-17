import { createContext, useContext, useState, type ReactNode } from 'react';
import type { AppState, MaterialTab } from '../types';

interface AppContextType {
  appState: AppState;
  setAppState: (state: AppState) => void;
  sourceLanguage: string;
  setSourceLanguage: (lang: string) => void;
  targetLanguage: string;
  setTargetLanguage: (lang: string) => void;
  selectedMaterialTab: MaterialTab;
  setSelectedMaterialTab: (tab: MaterialTab) => void;
  showSettings: boolean;
  setShowSettings: (show: boolean) => void;
  isMobile: boolean;
}

const AppContext = createContext<AppContextType | null>(null);

export function AppProvider({ children }: { children: ReactNode }) {
  const [appState, setAppState] = useState<AppState>({ status: 'idle' });
  const [sourceLanguage, setSourceLanguage] = useState('zh');
  const [targetLanguage, setTargetLanguage] = useState('en');
  const [selectedMaterialTab, setSelectedMaterialTab] = useState<MaterialTab>('video');
  const [showSettings, setShowSettings] = useState(false);
  const [isMobile] = useState(window.innerWidth < 768);

  const contextValue: AppContextType = {
    appState,
    setAppState,
    sourceLanguage,
    setSourceLanguage,
    targetLanguage,
    setTargetLanguage,
    selectedMaterialTab,
    setSelectedMaterialTab,
    showSettings,
    setShowSettings,
    isMobile,
  };

  return (
    <AppContext.Provider value={contextValue}>
      {children}
    </AppContext.Provider>
  );
}

export function useAppContext() {
  const context = useContext(AppContext);
  if (!context) {
    throw new Error('useAppContext must be used within an AppProvider');
  }
  return context;
}
