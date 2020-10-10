import React from "react";

export default ({ children }) => (
  <div
    style={{
      width: "100vw",
      height: "100vh",
      display: "flex",
      justifyContent: "center",
      alignItems: "center",
      fontSize: ".5rem",
    }}
  >
    {children}
  </div>
);
