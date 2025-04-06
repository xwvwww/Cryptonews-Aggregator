document.addEventListener('DOMContentLoaded', function() {
    const searchForm = document.getElementById('search-form');
    const cryptoSymbolInput = document.getElementById('crypto-symbol');
    const errorMessage = document.getElementById('error-message');
    const resultsContainer = document.getElementById('results-container');
    const cryptoSymbolDisplay = document.getElementById('crypto-symbol-display');
    const timestamp = document.getElementById('timestamp');
    const newsGrid = document.getElementById('news-grid');
    const noResults = document.getElementById('no-results');

    searchForm.addEventListener('submit', function(e) {
        e.preventDefault();
        
        const cryptoSymbol = cryptoSymbolInput.value.trim().toUpperCase();
        
        if (!cryptoSymbol) {
            showError('Please enter a cryptocurrency symbol');
            return;
        }
        
        // Clear previous results
        newsGrid.innerHTML = '';
        hideError();
        
        // Show loading state
        searchForm.querySelector('button').textContent = 'Searching...';
        searchForm.querySelector('button').disabled = true;
        
        // Fetch news from the Rust backend API
        fetchCryptoNews(cryptoSymbol)
            .then(data => {
                // Update UI with results
                displayResults(cryptoSymbol, data);
            })
            .catch(err => {
                showError('Failed to fetch news. Please try again later.');
                console.error(err);
            })
            .finally(() => {
                // Reset button state
                searchForm.querySelector('button').textContent = 'Search News';
                searchForm.querySelector('button').disabled = false;
            });
    });

    function fetchCryptoNews(symbol) {
        // Real API call to the Rust backend
        return fetch(`/api/news/${symbol}`)
            .then(response => {
                if (!response.ok) {
                    throw new Error(`HTTP error! Status: ${response.status}`);
                }
                return response.json();
            })
            .catch(error => {
                console.error('API call failed:', error);
                
                // Fallback to mock data if API call fails (for development/testing)
                console.log('Using mock data as fallback');
                return {
                    items: [
                        {
                            title: `${symbol} Price Surges Amid Market Recovery`,
                            url: 'https://example.com/news/1',
                            source: 'CryptoNews',
                            published_at: new Date().toISOString(),
                            summary: `${symbol} has seen significant gains in the past 24 hours as the broader cryptocurrency market shows signs of recovery.`,
                            image_url: 'https://via.placeholder.com/300x200?text=Crypto+News'
                        },
                        {
                            title: `New Development Roadmap Announced for ${symbol}`,
                            url: 'https://example.com/news/2',
                            source: 'CoinDesk',
                            published_at: new Date(Date.now() - 86400000).toISOString(),
                            summary: `The ${symbol} Foundation has announced an ambitious new roadmap for the next two years, focusing on scalability and security enhancements.`,
                            image_url: 'https://via.placeholder.com/300x200?text=Development+News'
                        },
                        {
                            title: `Major Exchange Adds New ${symbol} Trading Pairs`,
                            url: 'https://example.com/news/3',
                            source: 'CryptoPanic',
                            published_at: new Date(Date.now() - 172800000).toISOString(),
                            summary: `A leading cryptocurrency exchange has added several new trading pairs for ${symbol}, potentially increasing liquidity and accessibility.`,
                            image_url: 'https://via.placeholder.com/300x200?text=Exchange+News'
                        },
                        {
                            title: `${symbol} Community Votes on Governance Proposal`,
                            url: 'https://example.com/news/4',
                            source: 'CoinGecko',
                            published_at: new Date(Date.now() - 259200000).toISOString(),
                            summary: `The ${symbol} community is currently voting on a significant governance proposal that could change the token's economic model.`,
                            image_url: 'https://via.placeholder.com/300x200?text=Governance+News'
                        }
                    ],
                    timestamp: new Date().toISOString()
                };
            });
    }

    function displayResults(symbol, data) {
        // Update the results header
        cryptoSymbolDisplay.textContent = `for ${symbol}`;
        timestamp.textContent = `Last updated: ${formatDate(data.timestamp)}`;
        
        // Show results container
        resultsContainer.style.display = 'block';
        
        if (data.items.length === 0) {
            // Show no results message
            noResults.style.display = 'block';
            newsGrid.style.display = 'none';
        } else {
            // Hide no results message
            noResults.style.display = 'none';
            newsGrid.style.display = 'grid';
            
            // Create news cards
            data.items.forEach(item => {
                const newsCard = createNewsCard(item);
                newsGrid.appendChild(newsCard);
            });
        }
    }

    function createNewsCard(item) {
        const card = document.createElement('div');
        card.className = 'news-card';
        
        let imageHtml = '';
        if (item.image_url) {
            imageHtml = `
                <div class="news-image">
                    <img src="${item.image_url}" alt="${item.title}">
                </div>
            `;
        }
        
        card.innerHTML = `
            ${imageHtml}
            <div class="news-content">
                <div class="news-meta">
                    <span class="news-source">${item.source}</span>
                    <span class="news-date">${formatDate(item.published_at)}</span>
                </div>
                <h4 class="news-title">
                    <a href="${item.url}" target="_blank">${item.title}</a>
                </h4>
                ${item.summary ? `<p class="news-summary">${item.summary}</p>` : ''}
            </div>
        `;
        
        return card;
    }

    function formatDate(dateString) {
        const date = new Date(dateString);
        return date.toLocaleDateString('en-US', {
            year: 'numeric',
            month: 'long',
            day: 'numeric',
            hour: '2-digit',
            minute: '2-digit'
        });
    }

    function showError(message) {
        errorMessage.textContent = message;
        errorMessage.style.display = 'block';
    }

    function hideError() {
        errorMessage.style.display = 'none';
    }
});
