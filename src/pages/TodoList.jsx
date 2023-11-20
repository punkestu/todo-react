import { useState } from "react";
import Todo from "../components/Todo";

function TodoList() {
  const [todos, setTodos] = useState([
    {
      id: 1,
      label:
        "Lorem ipsum, dolor sit amet consectetur adipisicing elit. Voluptates, ex",
      state: false,
    },
    { id: 2, label: "Voluptates, ex", state: false },
    { id: 3, label: "dolor sit amet", state: false },
    { id: 4, label: "ipsum dolor", state: false },
    { id: 5, label: "consectetur", state: false },
    { id: 6, label: "adipisicing elit", state: false },
    { id: 7, label: "sit amet consectetur", state: false },
  ]);
  return (
    <div className="flex flex-col items-center justify-center w-3/4 h-screen p-4">
      <h1 className="font-bold text-2xl text-center mb-2">Todo List</h1>
      <section className="flex items-start justify-center flex-wrap gap-2">
        {todos.length > 0 ? (
          todos.map((todo) => {
            return (
              <Todo
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

export default TodoList;
