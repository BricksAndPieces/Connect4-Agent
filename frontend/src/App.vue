<script setup lang="ts">
import ConnectFour from './components/ConnectFour.vue'
</script>

<template>
  <div class="page-container">
    <div style="width: 570px">
      <h1><span>Connect 4 Agent</span></h1>
      <p>
        With over 4 trillion possible board states, Connect 4 is a suprisingly complex game that can
        be strongly solved. This agent uses the minimax algorithm to find the best move at every
        board state. However, the player going first has an advantage which makes it possible to
        force a win as player one.
      </p>
      <p style="margin-top: 10px">Can you beat the agent?</p>
      <br />
      <ConnectFour />
      <br />
      <h2><span>Algorithm</span></h2>
      <p>
        The agent utilizes the minimax algorithm in order to play optimally. The minimax algorithm
        is a recursive algorithm that is able to find the optimal move for a player assuming that
        the opponent plays optimally as well. The algorithm works by recursively exploring the child
        nodes of the current board state until it reaches a terminal state (such as a win or draw).
      </p>
      <br />
      <h3><span>Optimizations</span></h3>
      <p>
        There are 4,531,985,219,092 different possible board states in Connect 4. This is far too
        many to explore in a reasonable amount of time. In order to reduce the number of states that
        need to be explored and improve the performance of the algorithm, we can use a few
        optimizations.
      </p>
      <br />
      <ul>
        <li>
          <h4><span>Alpha-Beta Pruning</span></h4>
          <p>
            Reduce the number of nodes that need to be explored by pruning branches that are
            guaranteed to result in a worse best move than the current best move.
          </p>
        </li>
        <li>
          <h4><span>Move Ordering</span></h4>
          <p>
            Improve the performance of alpha-beta pruning by exploring the (assumed) best moves
            first.
          </p>
        </li>
        <li>
          <h4><span>Transposition Tables</span></h4>
          <p>
            Avoid exploring the same node multiple times by storing the results of previously
            explored nodes.
          </p>
        </li>
        <li>
          <h4><span>Iterative Deepening</span></h4>
          <p>
            Explore deeper into the game tree without sacrificing performance by running the
            algorithm multiple times with an increasing depth limit.
          </p>
        </li>
        <li>
          <h4><span>Opening Database</span></h4>
          <p>
            Avoid exploring the first few moves of the game by storing the best move at all possible
            early game board states.
          </p>
        </li>
      </ul>
      <br />
      <h2><span>Implementation</span></h2>
      <p>
        The agent was implemented in rust and then made accessible using a simple backend API. The
        frontend was implemented using Vue.js and communicates with the backend API using HTTP
        requests. This allowed the benefits of rust (speed) to be used while still allowing for a
        nice looking frontend. In order to make running the project as easy as possible, the project
        was containerized using Docker.
      </p>
      <!-- <br />
      <p>View the project on <a href="https://example.com" target="_blank">GitHub</a>.</p> -->
      <footer>
        <p>Made with Rust and Vue.js by Ishan Shetty</p>
      </footer>
    </div>
  </div>
</template>

<style scoped lang="scss">
.page-container {
  display: flex;
  align-items: center;
  flex-direction: column;
  min-height: 100vh;
  padding: 2rem;
  box-sizing: border-box;

  ul {
    padding-left: 20px;

    li:not(:last-child) {
      margin-bottom: 10px;
    }
  }

  footer {
    text-align: center;
    margin-top: 50px;
    font-size: 12px;
    color: var(--color-highlight);
  }
}
</style>
