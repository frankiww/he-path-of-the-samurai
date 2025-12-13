<!doctype html>
<html lang="ru">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Space Dashboard</title>
  <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/css/bootstrap.min.css" rel="stylesheet">
  <link rel="stylesheet" href="https://unpkg.com/leaflet@1.9.4/dist/leaflet.css"/>
  <style>
    #map{height:340px}
    body { padding-top: 70px; }

    /* Навбар кнопки */
    .navbar-nav .nav-link {
      margin-right: 0.5rem;
      padding: 0.5rem 1rem;
      border-radius: 0.25rem;
      transition: background 0.2s;
    }

    .navbar-nav .nav-link:hover {
      background-color: #495057; /* чуть светлее темного фона */
    }

    .navbar-brand {
      font-weight: bold;
    }

    .navbar {
      box-shadow: 0 2px 6px rgba(0,0,0,0.4);
    }
  </style>
  <script src="https://unpkg.com/leaflet@1.9.4/dist/leaflet.js"></script>
  <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
</head>
<body class="bg-dark text-light">
<nav class="navbar navbar-expand-lg navbar-dark bg-dark fixed-top">
  <div class="container">
    <div class="collapse navbar-collapse">
      <ul class="navbar-nav me-auto mb-2 mb-lg-0">
        <li class="nav-item">
          <a class="nav-link text-light" href="/dashboard">Dashboard</a>
        </li>
        <li class="nav-item">
          <a class="nav-link text-light" href="/iss">ISS</a>
        </li>
        <li class="nav-item">
          <a class="nav-link text-light" href="/osdr">OSDR</a>
        </li>
      </ul>
    </div>
  </div>
</nav>
@yield('content')
<script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/js/bootstrap.bundle.min.js"></script>
</body>
</html>
