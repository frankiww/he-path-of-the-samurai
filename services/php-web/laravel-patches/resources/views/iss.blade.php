@extends('layouts.app')

@section('content')
<div class="container py-4">
  <h3 class="mb-3 text-light">МКС данные</h3>

  <div class="row g-3">
    <div class="col-md-6">
      <div class="card bg-dark text-light shadow-sm border-secondary">
        <div class="card-body">
          <h5 class="card-title">Последний снимок</h5>
          @if(!empty($last['payload']))
            <ul class="list-group list-group-flush bg-dark text-light border-secondary">
              <li class="list-group-item bg-secondary text-light border-secondary">Широта {{ $last['payload']['latitude'] ?? '—' }}</li>
              <li class="list-group-item bg-secondary text-light border-secondary">Долгота {{ $last['payload']['longitude'] ?? '—' }}</li>
              <li class="list-group-item bg-secondary text-light border-secondary">Высота км {{ $last['payload']['altitude'] ?? '—' }}</li>
              <li class="list-group-item bg-secondary text-light border-secondary">Скорость км/ч {{ $last['payload']['velocity'] ?? '—' }}</li>
              <li class="list-group-item bg-secondary text-light border-secondary">Время {{ $last['fetched_at'] ?? '—' }}</li>
            </ul>
          @else
            <div class="text-muted">нет данных</div>
          @endif
          <div class="mt-3"><code class="text-light">{{ $base }}/last</code></div>
        </div>
      </div>
    </div>

    <div class="col-md-6">
      <div class="card bg-dark text-light shadow-sm border-secondary">
        <div class="card-body">
          <h5 class="card-title">Тренд движения</h5>
          @if(!empty($trend))
            <ul class="list-group list-group-flush bg-dark text-light border-secondary">
              <li class="list-group-item bg-secondary text-light border-secondary">Движение: {{ ($trend['movement'] ?? false) ? 'да' : 'нет' }}</li>
              <li class="list-group-item bg-secondary text-light border-secondary">Смещение: {{ number_format($trend['delta_km'] ?? 0, 3, '.', ' ') }} км</li>
              <li class="list-group-item bg-secondary text-light border-secondary">Интервал: {{ $trend['dt_sec'] ?? 0 }} сек</li>
              <li class="list-group-item bg-secondary text-light border-secondary">Скорость: {{ $trend['velocity_kmh'] ?? '—' }} км/ч</li>
            </ul>
          @else
            <div class="text-light">нет данных</div>
          @endif
          <div class="mt-3"><code class="text-light">{{ $base }}/iss/trend</code></div>
          <div class="mt-3">
            <a class="btn btn-outline-light" href="/osdr">Перейти к OSDR</a>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>
@endsection
