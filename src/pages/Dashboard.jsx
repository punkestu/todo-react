import { invoke } from "@tauri-apps/api";
import { useState } from "react";
import RecentTodos from "../components/RecentTodos";

function Dashboard() {
  const [label, setLabel] = useState("");
  const [event, setEvent] = useState("");
  return (
    <section className="flex-grow h-screen flex flex-col gap-4 justify-center items-center">
      <form
        onSubmit={(e) => {
          e.preventDefault();
          invoke("create_todo", {label}).then(_=>{
            setEvent("todo.created");
            setLabel("");
          })
        }}
        className="flex flex-col items-center w-full gap-y-2"
      >
        <h1 className="font-bold text-2xl">Create New Todo</h1>
        <input
          type="text"
          placeholder="Todo label"
          className="text-center w-1/2 border-2 border-slate-500 active:border-slate-900 px-4 py-1 rounded-lg"
          name="label"
          id="label"
          value={label}
          onChange={(e) => setLabel(e.target.value)}
        />
        <button
          type="submit"
          className="w-1/2 bg-blue-700 text-slate-200 px-6 py-1 rounded-lg text-lg hover:bg-blue-500 hover:text-slate-300 duration-200"
        >
          Create
        </button>
      </form>
      <RecentTodos event={event} setEvent={setEvent}/>
    </section>
  );
}

export default Dashboard;
