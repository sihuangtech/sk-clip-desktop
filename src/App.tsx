import { AppProvider } from './context/AppContext';
import { EditorHeader } from './components/layout/EditorHeader';
import { MaterialsPanel } from './components/materials/MaterialsPanel';
import { PreviewArea } from './components/preview/PreviewArea';
import { TranslationPanel } from './components/translation/TranslationPanel';
import { ProjectTimeline } from './components/timeline/ProjectTimeline';
import { SettingsPanel } from './components/settings/SettingsPanel';
import './App.css';

function App() {
  return (
    <AppProvider>
      <div className="app">
        <EditorHeader />
        <main className="editor-body">
          <MaterialsPanel />
          <PreviewArea />
          <TranslationPanel />
        </main>
        <ProjectTimeline />
        <SettingsPanel />
      </div>
    </AppProvider>
  );
}

export default App;
