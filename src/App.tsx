import { ShelfBar } from "./components/ShelfBar";
import { DropZone } from "./components/DropZone";
import { SettingsPanel } from "./components/Settings";
import { useHotzoneState } from "./hooks/useHotzoneState";
import "./App.css";

function App() {
  const { isVisible } = useHotzoneState();

  return (
    <div className="app" data-visible={isVisible}>
      <DropZone>
        <ShelfBar />
      </DropZone>
      <SettingsPanel />
    </div>
  );
}

export default App;
