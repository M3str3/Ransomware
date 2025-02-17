import React, { useState, useEffect } from 'react';
import './Posts.css';

const Posts = () => {
  const [posts, setPosts] = useState([]);

  useEffect(() => {
    fetch('http://m3str3qek4qgd2qlp2r6luqju35dts7vyt5n56n657z5hiwtfgnv62qd.onion/api/posts/')
      .then((res) => res.json())
      .then((data) => setPosts(data))
      .catch((err) => console.error('Error fetching posts:', err));
  }, []);

  return (
    <div className="page-container">
      <header className="header">
        <h1>☣️ M3str3 ☣️</h1>
        <nav className="nav-links">
          <a href="#posts">Attacks</a>
          <a href="#about">About</a>
          <a href="#contact">Contact</a>
        </nav>
      </header>
      <main className="main-content" id="posts">
        <section className="posts-section">
          <h2>Attacks</h2>
          {posts.length === 0 ? (
            <p>No posts yet....</p>
          ) : (
            <div className="posts-grid">
              {posts.map((post) => (
                <div key={post.id} className="post-card">
                  <h3>{post.name}</h3>
                  <p>{post.description}</p>
                  <p>
                    <strong>Price:</strong> {post.price || 'No price'}
                  </p>
                  <p>
                    <strong>Published:</strong> {post.published ? 'Yes' : 'No'}
                  </p>
                </div>
              ))}
            </div>
          )}
        </section>
      </main>
      <footer className="footer" id="contact">
        <p>© {new Date().getFullYear()} Example Ransom Blog (NOT REAL). No rights reserved.</p>
        <p>Contact me: namestre3@protonmail.com</p>
      </footer>
    </div>
  );
};

export default Posts;
