import { invoke } from "@tauri-apps/api";
import { useState, useEffect } from "react";
import ArchivedTodo from "../components/ArchivedTodo";

function ArchiveList() {
  const [todos, setTodos] = useState([]);
  useEffect(() => {
    invoke("get_archive").then((res) => {
      setTodos(res);
    });
  }, []);
  return (
    <div className="flex flex-col items-center justify-center w-3/4 h-screen p-4">
      <h1 className="font-bold text-2xl text-center mb-2">Archived</h1>
      <section className="flex items-start justify-center flex-wrap gap-2">
        {todos.length > 0 ? (
          todos.map((todo) => {
            return (
              <ArchivedTodo
                key={todo.id}
                todo={todo}
                todos={todos}
                setTodos={setTodos}
              />
            );
          })
        ) : (
          <p className="text-slate-400">no recent todo</p>
        )}
      </section>
    </div>
  );
}

export default ArchiveList;
