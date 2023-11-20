// import { invoke } from "@tauri-apps/api/tauri";
import { useState } from "react";
import "./App.css";
import Sidebar from "./components/Sidebar";
import Dashboard from "./pages/Dashboard";
import TodoList from "./pages/TodoList";
import ArchiveList from "./pages/Archive";

function App() {
  const [page, setPage] = useState("dashboard");
  return (
    <div className="flex items-start">
      <Sidebar page={page} setPage={setPage} />
      {(() => {
        switch (page) {
          case "dashboard":
            return <Dashboard />;
          case "list":
            return <TodoList />;
          case "archive":
            return <ArchiveList />;
          default:
            return <div>404</div>;
        }
      })()}
    </div>
  );
}

export default App;
