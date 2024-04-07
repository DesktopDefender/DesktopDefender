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
    <nav className="fixed top-0 left-0 bg-base-100 h-screen z-10">
      <div className="drawer h-screen flex pt-8 justify-center w-20">
        <input id="my-drawer" type="checkbox" className="drawer-toggle" />
        <div className="drawer-content">
          <div className="fixed border-DDOrange top-0 left-0 border-t-2 w-screen" />
          <label
            htmlFor="my-drawer" className="drawer-button flex flex-col justify-between w-6 h-5"
          >
            <span className="block w-full h-0.5 bg-white rounded" />
            <span className="block w-full h-0.5 bg-white rounded" />
            <span className="block w-full h-0.5 bg-white rounded" />
          </label>

        </div>

        <div className="drawer-side">
          <label
            htmlFor="my-drawer"
            aria-label="close sidebar"
            className="drawer-overlay"
          />
          <ul className="menu p-4 w-52 min-h-full bg-base-200 text-base-content">
            {renderItems()}
          </ul>
        </div>
      </div>
    </nav>
  );
}
