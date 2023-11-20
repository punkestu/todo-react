function tickTodo(todos, id) {
  return todos.map((todo) => {
    if (todo.id === id) {
      todo.state = true;
    }
    return todo;
  });
}

function Todo({ todo, todos, setTodos }) {
  return (
    <button
      className={`${todo.state ? "line-through animate-fade" : "underline hover:bg-white hover:drop-shadow-md duration-100"} px-1`}
      onClick={() => {
        setTodos(tickTodo(todos, todo.id));
        if (todo.state) {
          setTimeout(() => {
            todos = todos.filter((t) => t.id !== todo.id);
            setTodos(todos);
          }, 500);
        }
      }}
    >
      {todo.state && <span className="me-1">✅️</span>}
      {todo.label}
    </button>
  );
}

export default Todo;
