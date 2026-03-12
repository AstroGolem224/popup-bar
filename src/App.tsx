import { ShelfBar } from "./components/ShelfBar";
import { DropZone } from "./components/DropZone";
import { useHotzoneState } from "./hooks/useHotzoneState";
import "./App.css";

/**
 * Root application component.
 *
 * Wraps the ShelfBar in a DropZone and controls visibility
 * based on hotzone state. Phase 0: Always visible.
 */
function App() {
  const { isVisible } = useHotzoneState();

  return (
    <div className={`app ${isVisible ? "app--visible" : "app--hidden"}`}>
      <DropZone>
        <ShelfBar />
      </DropZone>
    </div>
  );
}

export default App;
