import Link from "next/link";

export default function Navbar() {
  return (
    <nav>
      <div className="drawer">
        <input id="my-drawer" type="checkbox" className="drawer-toggle" />
        <div className="drawer-content">
          <label htmlFor="my-drawer" className="btn btn-primary drawer-button">
            Open drawer
          </label>
        </div>

        <div className="drawer-side">
          <label
            htmlFor="my-drawer"
            aria-label="close sidebar"
            className="drawer-overlay"
          />
          <ul className="menu p-4 w-80 min-h-full bg-base-200 text-base-content">
            <li>
              <Link href="/">Home</Link>
            </li>
            <li>
              <Link href="/devices">Devices</Link>
            </li>
          </ul>
        </div>
      </div>
    </nav>
  );
}
