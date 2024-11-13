// pub async fn create_cart(
//     Path(user): Path<u32>,
//     State(state): State<AppState>,
//     Json(data): Json<cart_common::Cart>,
// ) -> Result<impl IntoResponse, AppError> {

//     let cart = Cart {
//         id: 0,           // primary key
//         user_cart_id: 0, // incremented by db based on user_id
//         user_id: user as i32,
//         discount_id,
//     };
//             diesel::insert_into(crate::schema::cart_lines::table)
//                 .values(lines)
//                 .execute(conn)?;
//             Ok::<i32, AppError>(user_cart_id)
//         })
//         .await??;
//     Ok(user_cart_id.to_string())
// }
