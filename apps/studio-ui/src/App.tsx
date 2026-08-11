import React, { useEffect } from 'react';
import { Header } from './components/layout/Header';
import { Sidebar } from './components/layout/Sidebar';
import { WorkspacePanel } from './components/layout/WorkspacePanel';
import { StatusBar } from './components/layout/StatusBar';
import { StudioIpcService } from './services/ipc.service';
import { useAppStore } from './stores/app.store';

export const App: React.FC = () => {
  const { setInitialized } = useAppStore();

  useEffect(() => {
    StudioIpcService.bootstrapStudio({
      enable_gpu_acceleration: true,
      developer_mode: false,
    }).then(() => {
      setInitialized(true);
    });
  }, [setInitialized]);

  return (
    <div className="flex flex-col h-screen w-screen overflow-hidden bg-background">
      <Header />
      <div className="flex flex-1 overflow-hidden">
        <Sidebar />
        <WorkspacePanel />
      </div>
      <StatusBar />
    </div>
  );
};

export default App;
