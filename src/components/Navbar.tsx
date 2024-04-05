import Link from "next/link";

const routes = [
  { href: "/", name: "Home" },
  { href: "/router", name: "Router" },
  { href: "/devices", name: "Devices" },
  { href: "/monitor", name: "Monitor" },
];

export default function Navbar() {
  const renderItems = () => {
    return routes.map((r) => (
      <li>
        <Link key={r.href} href={r.href}>
          {r.name}
        </Link>
      </li>
    ));
  };

  return (
    <nav className="absolute top-0 left-0 bg-slate-900">
      <div className="drawer h-screen flex items-center">
        <input id="my-drawer" type="checkbox" className="drawer-toggle" />
        <div className="drawer-content">
          <label htmlFor="my-drawer" className="btn btn-primary drawer-button">
            Sidebar
          </label>
        </div>

        <div className="drawer-side">
          <label
            htmlFor="my-drawer"
            aria-label="close sidebar"
            className="drawer-overlay"
          />
          <ul className="menu p-4 w-80 min-h-full bg-base-200 text-base-content">
            {renderItems()}
          </ul>
        </div>
      </div>
    </nav>
  );
}
