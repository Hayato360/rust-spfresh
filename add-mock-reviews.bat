@echo off
echo ========================================
echo  ADDING MOCK REVIEWS FOR DEMO
echo ========================================
echo.
echo This will add realistic sample reviews to test semantic search
echo Make sure the backend is running on http://localhost:8000
echo.
pause
echo.

echo Adding Laptop Reviews...
curl -X POST http://localhost:8000/reviews -H "Content-Type: application/json" -d "{\"product_id\":\"macbook-pro-16\",\"review_title\":\"Exceptional battery life and performance\",\"review_body\":\"This MacBook Pro has incredible battery life that easily lasts 12+ hours. The M2 chip delivers blazing fast performance for video editing and coding. Perfect for professionals.\",\"review_rating\":5}"

curl -X POST http://localhost:8000/reviews -H "Content-Type: application/json" -d "{\"product_id\":\"dell-xps-13\",\"review_title\":\"Beautiful display but average battery\",\"review_body\":\"The 4K OLED screen is absolutely stunning with vibrant colors and sharp text. However, the battery only lasts about 6 hours with moderate use. Great for design work.\",\"review_rating\":4}"

curl -X POST http://localhost:8000/reviews -H "Content-Type: application/json" -d "{\"product_id\":\"thinkpad-x1\",\"review_title\":\"Reliable workhorse with excellent keyboard\",\"review_body\":\"ThinkPad delivers solid performance and the keyboard is a joy to type on. Battery life is decent at 8-9 hours. Perfect for business travel and productivity.\",\"review_rating\":4}"

echo.
echo Adding Phone Reviews...
curl -X POST http://localhost:8000/reviews -H "Content-Type: application/json" -d "{\"product_id\":\"iphone-15-pro\",\"review_title\":\"Amazing camera quality and smooth performance\",\"review_body\":\"The camera system is phenomenal for photography and video recording. A17 Pro chip handles everything smoothly. Battery easily lasts all day with heavy usage.\",\"review_rating\":5}"

curl -X POST http://localhost:8000/reviews -H "Content-Type: application/json" -d "{\"product_id\":\"samsung-s24\",\"review_title\":\"Brilliant AMOLED screen and fast charging\",\"review_body\":\"The display is gorgeous with deep blacks and bright colors. 45W fast charging gets you to 80%% in 30 minutes. Great for multimedia consumption.\",\"review_rating\":4}"

curl -X POST http://localhost:8000/reviews -H "Content-Type: application/json" -d "{\"product_id\":\"pixel-8\",\"review_title\":\"Best Android camera but disappointing battery\",\"review_body\":\"Google's computational photography is unmatched. However, battery life is poor, barely lasting a full day with normal use. Frequent charging required.\",\"review_rating\":3}"

echo.
echo Adding Headphone Reviews...
curl -X POST http://localhost:8000/reviews -H "Content-Type: application/json" -d "{\"product_id\":\"sony-wh1000xm5\",\"review_title\":\"Industry-leading noise cancellation\",\"review_body\":\"The active noise cancellation is absolutely incredible for flights and commuting. Sound quality is rich and balanced. 30-hour battery life is impressive.\",\"review_rating\":5}"

curl -X POST http://localhost:8000/reviews -H "Content-Type: application/json" -d "{\"product_id\":\"airpods-pro-2\",\"review_title\":\"Convenient but expensive for what you get\",\"review_body\":\"The convenience factor is high with seamless Apple integration. Noise cancellation is good but not the best. Battery life of 6 hours is adequate.\",\"review_rating\":3}"

echo.
echo Adding Monitor Reviews...
curl -X POST http://localhost:8000/reviews -H "Content-Type: application/json" -d "{\"product_id\":\"lg-ultrawide-34\",\"review_title\":\"Perfect for productivity and gaming\",\"review_body\":\"The ultrawide 34-inch display transforms productivity with excellent multitasking. Great for gaming too with smooth 144Hz refresh rate. Colors are accurate.\",\"review_rating\":5}"

curl -X POST http://localhost:8000/reviews -H "Content-Type: application/json" -d "{\"product_id\":\"dell-4k-27\",\"review_title\":\"Sharp 4K but poor for gaming\",\"review_body\":\"The 4K resolution is crisp for text and photo editing. However, 60Hz refresh rate makes it unsuitable for competitive gaming. Good for office work.\",\"review_rating\":3}"

echo.
echo Adding Problematic Reviews...
curl -X POST http://localhost:8000/reviews -H "Content-Type: application/json" -d "{\"product_id\":\"budget-laptop\",\"review_title\":\"Slow performance and terrible build quality\",\"review_body\":\"This laptop is extremely slow even for basic tasks. The plastic build feels cheap and the keyboard is mushy. Battery dies in 3 hours. Avoid!\",\"review_rating\":1}"

curl -X POST http://localhost:8000/reviews -H "Content-Type: application/json" -d "{\"product_id\":\"cheap-phone\",\"review_title\":\"Frequent crashes and poor camera\",\"review_body\":\"The phone crashes multiple times daily and apps freeze constantly. Camera quality is terrible in low light. Very disappointing experience.\",\"review_rating\":2}"

echo.
echo ========================================
echo  MOCK REVIEWS ADDED SUCCESSFULLY!
echo ========================================
echo.
echo You now have 12 realistic reviews to test semantic search:
echo.
echo TRY THESE SEARCHES:
echo - "battery life" (find battery-related reviews)
echo - "camera quality" (find camera reviews)  
echo - "display screen" (find screen/monitor reviews)
echo - "slow performance" (find performance issues)
echo - "gaming" (find gaming-related reviews)
echo - "noise cancellation" (find headphone reviews)
echo.
echo Visit http://localhost:8080/search to test!
echo.
pause
