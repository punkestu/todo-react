function SidebarButton({ children, currPage, page, setPage }) {
  return (
    <button
      className={`border ${
        currPage === page ? "text-slate-300 fill-slate-300" : "border-transparent"
      } text-left px-2 py-1 rounded flex items-center gap-2`}
      onClick={() => setPage(page)}
    >
      {children}
    </button>
  );
}

export default SidebarButton;
