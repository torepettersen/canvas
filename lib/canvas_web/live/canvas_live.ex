defmodule CanvasWeb.CanvasLive do
  use CanvasWeb, :live_view

  def mount(_params, _session, socket) do
    {:ok, socket}
  end
end
