import { invoke } from "@tauri-apps/api";
import { useEffect, useState } from "react";
import RecentTodo from "./RecentTodo";

function RecentTodos({ event, setEvent }) {
  const [todos, setTodos] = useState({});
  useEffect(() => {
    invoke("get_recent").then(res=>{
      if(res){
        setTodos([res]);
      }
    });
  }, []);
  useEffect(() => {
    if (event === "todo.created") {
      invoke("get_recent").then(res=>{
        if(res){
          setTodos([res]);
        }
        setEvent("");
      });
    }
  }, [event]);
  return (
    <div className="flex flex-col items-center gap-1">
      <h2 className="font-bold text-lg text-center">Recent Todo</h2>
      <section className="flex flex-col">
        {todos.length > 0 ? (
          todos.slice(-1).map((todo) => {
            return (
              <RecentTodo
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

export default RecentTodos;
